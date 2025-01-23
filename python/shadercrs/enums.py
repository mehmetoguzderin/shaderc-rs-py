from enum import Enum


class TargetEnv(Enum):
    VULKAN = "vulkan"
    OPENGL = "opengl"
    OPENGL_COMPAT = "opengl_compat"


class EnvVersion(Enum):
    VULKAN_1_0 = "vulkan_1_0"
    VULKAN_1_1 = "vulkan_1_1"
    VULKAN_1_2 = "vulkan_1_2"
    VULKAN_1_3 = "vulkan_1_3"
    OPENGL_4_5 = "opengl_4_5"


class SpirvVersion(Enum):
    V_1_0 = "v_1_0"
    V_1_1 = "v_1_1"
    V_1_2 = "v_1_2"
    V_1_3 = "v_1_3"
    V_1_4 = "v_1_4"
    V_1_5 = "v_1_5"
    V_1_6 = "v_1_6"


class SourceLanguage(Enum):
    GLSL = "glsl"
    HLSL = "hlsl"


class ResourceKind(Enum):
    IMAGE = "image"
    SAMPLER = "sampler"
    TEXTURE = "texture"
    BUFFER = "buffer"
    STORAGE_BUFFER = "storage_buffer"
    UNORDERED_ACCESS_VIEW = "unordered_access_view"


class ShaderKind(Enum):
    VERTEX = "vertex"
    FRAGMENT = "fragment"
    COMPUTE = "compute"
    GEOMETRY = "geometry"
    TESS_CONTROL = "tess_control"
    TESS_EVALUATION = "tess_evaluation"
    INFER_FROM_SOURCE = "infer_from_source"
    DEFAULT_VERTEX = "default_vertex"
    DEFAULT_FRAGMENT = "default_fragment"
    DEFAULT_COMPUTE = "default_compute"
    DEFAULT_GEOMETRY = "default_geometry"
    DEFAULT_TESS_CONTROL = "default_tess_control"
    DEFAULT_TESS_EVALUATION = "default_tess_evaluation"
    SPIRV_ASSEMBLY = "spirv_assembly"
    RAY_GENERATION = "ray_generation"
    ANY_HIT = "any_hit"
    CLOSEST_HIT = "closest_hit"
    MISS = "miss"
    INTERSECTION = "intersection"
    CALLABLE = "callable"
    DEFAULT_RAY_GENERATION = "default_ray_generation"
    DEFAULT_ANY_HIT = "default_any_hit"
    DEFAULT_CLOSEST_HIT = "default_closest_hit"
    DEFAULT_MISS = "default_miss"
    DEFAULT_INTERSECTION = "default_intersection"
    DEFAULT_CALLABLE = "default_callable"
    TASK = "task"
    MESH = "mesh"
    DEFAULT_TASK = "default_task"
    DEFAULT_MESH = "default_mesh"


class GlslProfile(Enum):
    NONE = "none"
    CORE = "core"
    COMPATIBILITY = "compatibility"
    ES = "es"


class OptimizationLevel(Enum):
    ZERO = "zero"
    SIZE = "size"
    PERFORMANCE = "performance"


class Limit(Enum):
    MAX_LIGHTS = "max_lights"
    MAX_CLIP_PLANES = "max_clip_planes"
    MAX_TEXTURE_UNITS = "max_texture_units"
    MAX_TEXTURE_COORDS = "max_texture_coords"
    MAX_VERTEX_ATTRIBS = "max_vertex_attribs"
    MAX_VERTEX_UNIFORM_COMPONENTS = "max_vertex_uniform_components"
    MAX_VARYING_FLOATS = "max_varying_floats"
    MAX_VERTEX_TEXTURE_IMAGE_UNITS = "max_vertex_texture_image_units"
    MAX_COMBINED_TEXTURE_IMAGE_UNITS = "max_combined_texture_image_units"
    MAX_TEXTURE_IMAGE_UNITS = "max_texture_image_units"
    MAX_FRAGMENT_UNIFORM_COMPONENTS = "max_fragment_uniform_components"
    MAX_DRAW_BUFFERS = "max_draw_buffers"
    MAX_VERTEX_UNIFORM_VECTORS = "max_vertex_uniform_vectors"
    MAX_VARYING_VECTORS = "max_varying_vectors"
    MAX_FRAGMENT_UNIFORM_VECTORS = "max_fragment_uniform_vectors"
    MAX_VERTEX_OUTPUT_VECTORS = "max_vertex_output_vectors"
    MAX_FRAGMENT_INPUT_VECTORS = "max_fragment_input_vectors"
    MIN_PROGRAM_TEXEL_OFFSET = "min_program_texel_offset"
    MAX_PROGRAM_TEXEL_OFFSET = "max_program_texel_offset"
    MAX_CLIP_DISTANCES = "max_clip_distances"
    MAX_COMPUTE_WORK_GROUP_COUNT_X = "max_compute_work_group_count_x"
    MAX_COMPUTE_WORK_GROUP_COUNT_Y = "max_compute_work_group_count_y"
    MAX_COMPUTE_WORK_GROUP_COUNT_Z = "max_compute_work_group_count_z"
    MAX_COMPUTE_WORK_GROUP_SIZE_X = "max_compute_work_group_size_x"
    MAX_COMPUTE_WORK_GROUP_SIZE_Y = "max_compute_work_group_size_y"
    MAX_COMPUTE_WORK_GROUP_SIZE_Z = "max_compute_work_group_size_z"
    MAX_COMPUTE_UNIFORM_COMPONENTS = "max_compute_uniform_components"
    MAX_COMPUTE_TEXTURE_IMAGE_UNITS = "max_compute_texture_image_units"
    MAX_COMPUTE_IMAGE_UNIFORMS = "max_compute_image_uniforms"
    MAX_COMPUTE_ATOMIC_COUNTERS = "max_compute_atomic_counters"
    MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS = "max_compute_atomic_counter_buffers"
    MAX_VARYING_COMPONENTS = "max_varying_components"
    MAX_VERTEX_OUTPUT_COMPONENTS = "max_vertex_output_components"
    MAX_GEOMETRY_INPUT_COMPONENTS = "max_geometry_input_components"
    MAX_GEOMETRY_OUTPUT_COMPONENTS = "max_geometry_output_components"
    MAX_FRAGMENT_INPUT_COMPONENTS = "max_fragment_input_components"
    MAX_IMAGE_UNITS = "max_image_units"
    MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS = (
        "max_combined_image_units_and_fragment_outputs"
    )
    MAX_COMBINED_SHADER_OUTPUT_RESOURCES = "max_combined_shader_output_resources"
    MAX_IMAGE_SAMPLES = "max_image_samples"
    MAX_VERTEX_IMAGE_UNIFORMS = "max_vertex_image_uniforms"
    MAX_TESS_CONTROL_IMAGE_UNIFORMS = "max_tess_control_image_uniforms"
    MAX_TESS_EVALUATION_IMAGE_UNIFORMS = "max_tess_evaluation_image_uniforms"
    MAX_GEOMETRY_IMAGE_UNIFORMS = "max_geometry_image_uniforms"
    MAX_FRAGMENT_IMAGE_UNIFORMS = "max_fragment_image_uniforms"
    MAX_COMBINED_IMAGE_UNIFORMS = "max_combined_image_uniforms"
    MAX_GEOMETRY_TEXTURE_IMAGE_UNITS = "max_geometry_texture_image_units"
    MAX_GEOMETRY_OUTPUT_VERTICES = "max_geometry_output_vertices"
    MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS = "max_geometry_total_output_components"
    MAX_GEOMETRY_UNIFORM_COMPONENTS = "max_geometry_uniform_components"
    MAX_GEOMETRY_VARYING_COMPONENTS = "max_geometry_varying_components"
    MAX_TESS_CONTROL_INPUT_COMPONENTS = "max_tess_control_input_components"
    MAX_TESS_CONTROL_OUTPUT_COMPONENTS = "max_tess_control_output_components"
    MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS = "max_tess_control_texture_image_units"
    MAX_TESS_CONTROL_UNIFORM_COMPONENTS = "max_tess_control_uniform_components"
    MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS = (
        "max_tess_control_total_output_components"
    )
    MAX_TESS_EVALUATION_INPUT_COMPONENTS = "max_tess_evaluation_input_components"
    MAX_TESS_EVALUATION_OUTPUT_COMPONENTS = "max_tess_evaluation_output_components"
    MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS = "max_tess_evaluation_texture_image_units"
    MAX_TESS_EVALUATION_UNIFORM_COMPONENTS = "max_tess_evaluation_uniform_components"
    MAX_TESS_PATCH_COMPONENTS = "max_tess_patch_components"
    MAX_PATCH_VERTICES = "max_patch_vertices"
    MAX_TESS_GEN_LEVEL = "max_tess_gen_level"
    MAX_VIEWPORTS = "max_viewports"
    MAX_VERTEX_ATOMIC_COUNTERS = "max_vertex_atomic_counters"
    MAX_TESS_CONTROL_ATOMIC_COUNTERS = "max_tess_control_atomic_counters"
    MAX_TESS_EVALUATION_ATOMIC_COUNTERS = "max_tess_evaluation_atomic_counters"
    MAX_GEOMETRY_ATOMIC_COUNTERS = "max_geometry_atomic_counters"
    MAX_FRAGMENT_ATOMIC_COUNTERS = "max_fragment_atomic_counters"
    MAX_COMBINED_ATOMIC_COUNTERS = "max_combined_atomic_counters"
    MAX_ATOMIC_COUNTER_BINDINGS = "max_atomic_counter_bindings"
    MAX_VERTEX_ATOMIC_COUNTER_BUFFERS = "max_vertex_atomic_counter_buffers"
    MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS = "max_tess_control_atomic_counter_buffers"
    MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS = (
        "max_tess_evaluation_atomic_counter_buffers"
    )
    MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS = "max_geometry_atomic_counter_buffers"
    MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS = "max_fragment_atomic_counter_buffers"
    MAX_COMBINED_ATOMIC_COUNTER_BUFFERS = "max_combined_atomic_counter_buffers"
    MAX_ATOMIC_COUNTER_BUFFER_SIZE = "max_atomic_counter_buffer_size"
    MAX_TRANSFORM_FEEDBACK_BUFFERS = "max_transform_feedback_buffers"
    MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS = (
        "max_transform_feedback_interleaved_components"
    )
    MAX_CULL_DISTANCES = "max_cull_distances"
    MAX_COMBINED_CLIP_AND_CULL_DISTANCES = "max_combined_clip_and_cull_distances"
    MAX_SAMPLES = "max_samples"


class IncludeType(Enum):
    RELATIVE = "relative"
    STANDARD = "standard"
