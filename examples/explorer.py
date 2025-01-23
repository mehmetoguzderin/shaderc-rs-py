# Inspired by and based on:
#     https://github.com/pygfx/wgpu-py/blob/11585940405c33932e3c448b19134d5e54da1e5f/examples/gui_qt_asyncio.py
#     https://www.shadertoy.com/view/Xds3zN

import time
import struct
import asyncio

from PySide6 import QtCore, QtWidgets, QtAsyncio
from wgpu.gui.qt import WgpuWidget
import wgpu
import shadercrs


def async_connect(signal, async_function):
    def proxy():
        return asyncio.ensure_future(async_function())

    signal.connect(proxy)


VERTEX_SHADER = r"""
#version 450 core

layout(location = 0) out vec2 vUV;

void main()
{
    vec2 positions[3] = vec2[3](
        vec2(-1.0, -1.0),
        vec2( 3.0, -1.0),
        vec2(-1.0,  3.0)
    );
    gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
    vUV = gl_Position.xy * 0.5 + 0.5;
}
"""

DEFAULT_FRAGMENT_SHADER = r"""
#version 450 core

layout(location = 0) in vec2 vUV;
layout(location = 0) out vec4 outColor;

layout(set = 0, binding = 0) uniform Uniforms {
    mat4 transform;
} u;

const int MAX_STEPS = 64;
const float MAX_DISTANCE = 100.0;
const float EPSILON = 0.0005;

mat2 rot2(float a) {
    float c = cos(a), s = sin(a);
    return mat2(c, -s, s, c);
}

vec3 rotateY(vec3 p, float a) {
    p.xz = rot2(a) * p.xz;
    return p;
}

float sdBox(vec3 p, vec3 b) {
    vec3 d = abs(p) - b;
    return length(max(d, 0.0)) + min(max(d.x, max(d.y, d.z)), 0.0);
}

float mapScene(vec3 p, float time) {
    p = rotateY(p, -time);
    return sdBox(p, vec3(0.5));
}

float raymarch(vec3 ro, vec3 rd, float time) {
    float t = 0.0;
    for(int i = 0; i < MAX_STEPS; i++) {
        vec3 p = ro + rd * t;
        float dist = mapScene(p, time);
        if (dist < EPSILON) {
            return t;
        }
        t += dist;
        if (t > MAX_DISTANCE) {
            return -1.0;
        }
    }
    return -1.0;
}

vec3 getBoxNormalLocal(vec3 p) {
    vec3 a = abs(p);
    vec3 s = sign(p);
    if (a.x > a.y && a.x > a.z) {
        return vec3(s.x, 0.0, 0.0);
    } else if (a.y > a.x && a.y > a.z) {
        return vec3(0.0, s.y, 0.0);
    } else {
        return vec3(0.0, 0.0, s.z);
    }
}

vec2 getBoxUVLocal(vec3 p, vec3 n) {
    if (abs(n.x) > 0.5) {
        return p.yz + 0.5;
    } else if (abs(n.y) > 0.5) {
        return p.xz + 0.5;
    } else {
        return p.xy + 0.5;
    }
}

void main()
{
    float time = u.transform[3][3];
    vec2 screenPos = vUV * 2.0 - 1.0;
    vec3 ro = vec3(0.0, 0.0, 3.0);
    vec3 rd = normalize(vec3(screenPos, -1.0));
    float hitDist = raymarch(ro, rd, time);
    if (hitDist < 0.0) {
        outColor = vec4(vUV, 1.0, 1.0);
        return;
    }
    vec3 p = ro + hitDist * rd;
    vec3 localPos = rotateY(p, -time);
    vec3 faceNLocal = getBoxNormalLocal(localPos);
    vec3 normalRemap = faceNLocal * 0.5 + 0.5;
    vec2 faceUV = getBoxUVLocal(localPos, faceNLocal);
    vec3 color = vec3(
        faceUV.x * normalRemap.x,
        faceUV.y * normalRemap.y,
        normalRemap.z
    );
    outColor = vec4(color, 1.0);
}
"""


class ExampleWidget(QtWidgets.QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("shaderc-rs-py Explorer")

        self.canvas = WgpuWidget()
        self.canvas.setFixedSize(400, 400)

        self.frag_editor = QtWidgets.QTextEdit()
        self.frag_editor.setPlainText(DEFAULT_FRAGMENT_SHADER)

        self.log_panel = QtWidgets.QTextEdit()
        self.log_panel.setReadOnly(True)

        self.optim_level_combo = QtWidgets.QComboBox()
        self.optim_level_combo.addItems(["performance", "size", "zero"])

        self.compile_button = QtWidgets.QPushButton("Compile")

        controls_layout = QtWidgets.QVBoxLayout()
        controls_layout.addWidget(QtWidgets.QLabel("Fragment Shader:"))
        controls_layout.addWidget(self.frag_editor, stretch=1)
        controls_layout.addWidget(QtWidgets.QLabel("Optimization Level:"))
        controls_layout.addWidget(self.optim_level_combo)
        controls_layout.addWidget(self.compile_button)
        controls_layout.addWidget(QtWidgets.QLabel("Log:"))
        controls_layout.addWidget(self.log_panel, stretch=1)

        main_layout = QtWidgets.QHBoxLayout()
        main_layout.addLayout(controls_layout, stretch=1)
        main_layout.addWidget(self.canvas, stretch=0)
        self.setLayout(main_layout)

        self.start_time = time.time()

        async_connect(self.compile_button.clicked, self.onCompileClicked)

        self.init_wgpu()

        self.timer = QtCore.QTimer()
        self.timer.timeout.connect(lambda: self.canvas.request_draw(self.draw_frame))
        self.timer.start(20)

    def log(self, msg: str):
        old_text = self.log_panel.toPlainText()
        new_text = old_text + "\n" + msg
        self.log_panel.setPlainText(new_text)
        self.log_panel.verticalScrollBar().setValue(
            self.log_panel.verticalScrollBar().maximum()
        )

    def init_wgpu(self):
        self.adapter = wgpu.gpu.request_adapter_sync(
            power_preference="high-performance"
        )
        self.device = self.adapter.request_device_sync()

        context = self.canvas.get_context("wgpu")
        self.render_texture_format = context.get_preferred_format(self.adapter)
        context.configure(device=self.device, format=self.render_texture_format)

        self.transform_buffer = self.device.create_buffer(
            size=64,
            usage=wgpu.BufferUsage.UNIFORM | wgpu.BufferUsage.COPY_DST,
        )

        self.bind_group_layout = self.device.create_bind_group_layout(
            entries=[
                {
                    "binding": 0,
                    "visibility": wgpu.ShaderStage.VERTEX | wgpu.ShaderStage.FRAGMENT,
                    "buffer": {"type": wgpu.BufferBindingType.uniform},
                }
            ]
        )

        self.bind_group = self.device.create_bind_group(
            layout=self.bind_group_layout,
            entries=[
                {
                    "binding": 0,
                    "resource": {
                        "buffer": self.transform_buffer,
                        "offset": 0,
                        "size": 64,
                    },
                }
            ],
        )

        self.pipeline_layout = self.device.create_pipeline_layout(
            bind_group_layouts=[self.bind_group_layout]
        )

        self.vert_shader_module = None
        try:
            compiler = shadercrs.Compiler()
            opts = shadercrs.CompileOptions()
            opts.set_source_language("glsl")
            opts.set_target_env("vulkan", "vulkan_1_1")
            opts.set_optimization_level("performance")

            vert_spv = compiler.compile_into_spirv(
                source=VERTEX_SHADER,
                stage="vertex",
                entry_point="main",
                input_file_name="shader.vert",
                options=opts,
            )
            self.vert_shader_module = self.device.create_shader_module(
                code=vert_spv.as_binary_u8()
            )
        except Exception as e:
            self.log(f"Vertex shader compilation error: {e}")

        self.current_pipeline = None

        self.new_code = None
        self.new_code_exists = False

        self.new_code, error = self.build_code_from_fragment(
            self.frag_editor.toPlainText(), "performance"
        )

        if self.new_code:
            self.log("Initial compilation succeeded! Will swap next frame.")
            self.new_code_exists = True
        else:
            self.log(f"Initial compilation failed:\n{error}")

    def build_code_from_fragment(self, frag_code: str, optim_choice: str):
        if not self.vert_shader_module:
            return None, "No valid vertex shader module."
        try:
            compiler = shadercrs.Compiler()
            opts = shadercrs.CompileOptions()
            opts.set_source_language("glsl")
            opts.set_target_env("vulkan", "vulkan_1_1")

            if optim_choice == "performance":
                opts.set_optimization_level("performance")
            elif optim_choice == "size":
                opts.set_optimization_level("size")
            else:
                opts.set_optimization_level("none")

            frag_spv = compiler.compile_into_spirv(
                source=frag_code,
                stage="fragment",
                entry_point="main",
                input_file_name="shader.frag",
                options=opts,
            )
            new_code = frag_spv.as_binary_u8()
            return new_code, None
        except Exception as e:
            return None, str(e)

    async def onCompileClicked(self):
        frag_code = self.frag_editor.toPlainText()
        optim_choice = self.optim_level_combo.currentText()
        self.log(f"Compiling fragment (optim={optim_choice})...")

        new_code, error = await asyncio.to_thread(
            self.build_code_from_fragment, frag_code, optim_choice
        )
        if new_code:
            self.log("Compilation succeeded! Will swap next frame.")
            self.new_code = new_code
            self.new_code_exists = True
        else:
            self.log(f"Compilation failed:\n{error}")

    def draw_frame(self):
        if self.new_code_exists and self.new_code:
            self.log("Swapping in newly compiled code.")
            frag_mod = self.device.create_shader_module(code=self.new_code)
            pipeline_desc = {
                "layout": self.pipeline_layout,
                "vertex": {
                    "module": self.vert_shader_module,
                    "entry_point": "main",
                },
                "fragment": {
                    "module": frag_mod,
                    "entry_point": "main",
                    "targets": [
                        {
                            "format": self.render_texture_format,
                            "blend": {
                                "color": {},
                                "alpha": {},
                            },
                        },
                    ],
                },
                "primitive": {"topology": wgpu.PrimitiveTopology.triangle_list},
            }
            self.current_pipeline = self.device.create_render_pipeline(**pipeline_desc)
            self.new_code = None
            self.new_code_exists = False

        if not self.current_pipeline:
            return

        elapsed = time.time() - self.start_time
        matrix_data = [
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
            0.0,
            0.0,
            0.0,
            0.0,
            float(elapsed),
        ]
        self.device.queue.write_buffer(
            self.transform_buffer, 0, struct.pack("16f", *matrix_data)
        )

        ctx = self.canvas.get_context("wgpu")
        curr_tex = ctx.get_current_texture()
        encoder = self.device.create_command_encoder()

        rp = encoder.begin_render_pass(
            color_attachments=[
                {
                    "view": curr_tex.create_view(),
                    "resolve_target": None,
                    "clear_value": (0, 0, 0, 1),
                    "load_op": wgpu.LoadOp.clear,
                    "store_op": wgpu.StoreOp.store,
                }
            ]
        )
        rp.set_pipeline(self.current_pipeline)
        rp.set_bind_group(0, self.bind_group)
        rp.draw(3, 1, 0, 0)
        rp.end()
        self.device.queue.submit([encoder.finish()])


if __name__ == "__main__":
    app = QtWidgets.QApplication([])
    widget = ExampleWidget()
    widget.show()

    QtAsyncio.run()
