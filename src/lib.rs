use pyo3::prelude::*;

use shaderc;

pub trait FromPythonicString {
    fn from_pythonic_string(s: &str) -> Result<Self, String>
    where
        Self: Sized;
}

impl FromPythonicString for shaderc::TargetEnv {
    fn from_pythonic_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "vulkan" => Ok(Self::Vulkan),
            "opengl" => Ok(Self::OpenGL),
            "opengl_compat" => Ok(Self::OpenGLCompat),
            _ => Err(format!("Invalid value: {}", s)),
        }
    }
}

impl FromPythonicString for shaderc::EnvVersion {
    fn from_pythonic_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "vulkan_1_0" => Ok(Self::Vulkan1_0),
            "vulkan_1_1" => Ok(Self::Vulkan1_1),
            "vulkan_1_2" => Ok(Self::Vulkan1_2),
            "vulkan_1_3" => Ok(Self::Vulkan1_3),
            "opengl_4_5" => Ok(Self::OpenGL4_5),
            _ => Err(format!("Invalid value: {}", s)),
        }
    }
}

impl FromPythonicString for shaderc::SpirvVersion {
    fn from_pythonic_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "v_1_0" => Ok(Self::V1_0),
            "v_1_1" => Ok(Self::V1_1),
            "v_1_2" => Ok(Self::V1_2),
            "v_1_3" => Ok(Self::V1_3),
            "v_1_4" => Ok(Self::V1_4),
            "v_1_5" => Ok(Self::V1_5),
            "v_1_6" => Ok(Self::V1_6),
            _ => Err(format!("Invalid value: {}", s)),
        }
    }
}

impl FromPythonicString for shaderc::SourceLanguage {
    fn from_pythonic_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "glsl" => Ok(Self::GLSL),
            "hlsl" => Ok(Self::HLSL),
            _ => Err(format!("Invalid value: {}", s)),
        }
    }
}

impl FromPythonicString for shaderc::ResourceKind {
    fn from_pythonic_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "image" => Ok(Self::Image),
            "sampler" => Ok(Self::Sampler),
            "texture" => Ok(Self::Texture),
            "buffer" => Ok(Self::Buffer),
            "storage_buffer" => Ok(Self::StorageBuffer),
            "unordered_access_view" => Ok(Self::UnorderedAccessView),
            _ => Err(format!("Invalid value: {}", s)),
        }
    }
}

impl FromPythonicString for shaderc::ShaderKind {
    fn from_pythonic_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "vertex" => Ok(Self::Vertex),
            "fragment" => Ok(Self::Fragment),
            "compute" => Ok(Self::Compute),
            "geometry" => Ok(Self::Geometry),
            "tess_control" => Ok(Self::TessControl),
            "tess_evaluation" => Ok(Self::TessEvaluation),
            "infer_from_source" => Ok(Self::InferFromSource),
            "default_vertex" => Ok(Self::DefaultVertex),
            "default_fragment" => Ok(Self::DefaultFragment),
            "default_compute" => Ok(Self::DefaultCompute),
            "default_geometry" => Ok(Self::DefaultGeometry),
            "default_tess_control" => Ok(Self::DefaultTessControl),
            "default_tess_evaluation" => Ok(Self::DefaultTessEvaluation),
            "spirv_assembly" => Ok(Self::SpirvAssembly),
            "ray_generation" => Ok(Self::RayGeneration),
            "any_hit" => Ok(Self::AnyHit),
            "closest_hit" => Ok(Self::ClosestHit),
            "miss" => Ok(Self::Miss),
            "intersection" => Ok(Self::Intersection),
            "callable" => Ok(Self::Callable),
            "default_ray_generation" => Ok(Self::DefaultRayGeneration),
            "default_any_hit" => Ok(Self::DefaultAnyHit),
            "default_closest_hit" => Ok(Self::DefaultClosestHit),
            "default_miss" => Ok(Self::DefaultMiss),
            "default_intersection" => Ok(Self::DefaultIntersection),
            "default_callable" => Ok(Self::DefaultCallable),
            "task" => Ok(Self::Task),
            "mesh" => Ok(Self::Mesh),
            "default_task" => Ok(Self::DefaultTask),
            "default_mesh" => Ok(Self::DefaultMesh),
            _ => Err(format!("Invalid value: {}", s)),
        }
    }
}

impl FromPythonicString for shaderc::GlslProfile {
    fn from_pythonic_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "core" => Ok(Self::Core),
            "compatibility" => Ok(Self::Compatibility),
            "es" => Ok(Self::Es),
            _ => Err(format!("Invalid value: {}", s)),
        }
    }
}

impl FromPythonicString for shaderc::OptimizationLevel {
    fn from_pythonic_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "zero" => Ok(Self::Zero),
            "size" => Ok(Self::Size),
            "performance" => Ok(Self::Performance),
            _ => Err(format!("Invalid value: {}", s)),
        }
    }
}

impl FromPythonicString for shaderc::Limit {
    fn from_pythonic_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "max_lights" => Ok(Self::MaxLights),
            "max_clip_planes" => Ok(Self::MaxClipPlanes),
            "max_texture_units" => Ok(Self::MaxTextureUnits),
            "max_texture_coords" => Ok(Self::MaxTextureCoords),
            "max_vertex_attribs" => Ok(Self::MaxVertexAttribs),
            "max_vertex_uniform_components" => Ok(Self::MaxVertexUniformComponents),
            "max_varying_floats" => Ok(Self::MaxVaryingFloats),
            "max_vertex_texture_image_units" => Ok(Self::MaxVertexTextureImageUnits),
            "max_combined_texture_image_units" => Ok(Self::MaxCombinedTextureImageUnits),
            "max_texture_image_units" => Ok(Self::MaxTextureImageUnits),
            "max_fragment_uniform_components" => Ok(Self::MaxFragmentUniformComponents),
            "max_draw_buffers" => Ok(Self::MaxDrawBuffers),
            "max_vertex_uniform_vectors" => Ok(Self::MaxVertexUniformVectors),
            "max_varying_vectors" => Ok(Self::MaxVaryingVectors),
            "max_fragment_uniform_vectors" => Ok(Self::MaxFragmentUniformVectors),
            "max_vertex_output_vectors" => Ok(Self::MaxVertexOutputVectors),
            "max_fragment_input_vectors" => Ok(Self::MaxFragmentInputVectors),
            "min_program_texel_offset" => Ok(Self::MinProgramTexelOffset),
            "max_program_texel_offset" => Ok(Self::MaxProgramTexelOffset),
            "max_clip_distances" => Ok(Self::MaxClipDistances),
            "max_compute_work_group_count_x" => Ok(Self::MaxComputeWorkGroupCountX),
            "max_compute_work_group_count_y" => Ok(Self::MaxComputeWorkGroupCountY),
            "max_compute_work_group_count_z" => Ok(Self::MaxComputeWorkGroupCountZ),
            "max_compute_work_group_size_x" => Ok(Self::MaxComputeWorkGroupSizeX),
            "max_compute_work_group_size_y" => Ok(Self::MaxComputeWorkGroupSizeY),
            "max_compute_work_group_size_z" => Ok(Self::MaxComputeWorkGroupSizeZ),
            "max_compute_uniform_components" => Ok(Self::MaxComputeUniformComponents),
            "max_compute_texture_image_units" => Ok(Self::MaxComputeTextureImageUnits),
            "max_compute_image_uniforms" => Ok(Self::MaxComputeImageUniforms),
            "max_compute_atomic_counters" => Ok(Self::MaxComputeAtomicCounters),
            "max_compute_atomic_counter_buffers" => Ok(Self::MaxComputeAtomicCounterBuffers),
            "max_varying_components" => Ok(Self::MaxVaryingComponents),
            "max_vertex_output_components" => Ok(Self::MaxVertexOutputComponents),
            "max_geometry_input_components" => Ok(Self::MaxGeometryInputComponents),
            "max_geometry_output_components" => Ok(Self::MaxGeometryOutputComponents),
            "max_fragment_input_components" => Ok(Self::MaxFragmentInputComponents),
            "max_image_units" => Ok(Self::MaxImageUnits),
            "max_combined_image_units_and_fragment_outputs" => {
                Ok(Self::MaxCombinedImageUnitsAndFragmentOutputs)
            }
            "max_combined_shader_output_resources" => Ok(Self::MaxCombinedShaderOutputResources),
            "max_image_samples" => Ok(Self::MaxImageSamples),
            "max_vertex_image_uniforms" => Ok(Self::MaxVertexImageUniforms),
            "max_tess_control_image_uniforms" => Ok(Self::MaxTessControlImageUniforms),
            "max_tess_evaluation_image_uniforms" => Ok(Self::MaxTessEvaluationImageUniforms),
            "max_geometry_image_uniforms" => Ok(Self::MaxGeometryImageUniforms),
            "max_fragment_image_uniforms" => Ok(Self::MaxFragmentImageUniforms),
            "max_combined_image_uniforms" => Ok(Self::MaxCombinedImageUniforms),
            "max_geometry_texture_image_units" => Ok(Self::MaxGeometryTextureImageUnits),
            "max_geometry_output_vertices" => Ok(Self::MaxGeometryOutputVertices),
            "max_geometry_total_output_components" => Ok(Self::MaxGeometryTotalOutputComponents),
            "max_geometry_uniform_components" => Ok(Self::MaxGeometryUniformComponents),
            "max_geometry_varying_components" => Ok(Self::MaxGeometryVaryingComponents),
            "max_tess_control_input_components" => Ok(Self::MaxTessControlInputComponents),
            "max_tess_control_output_components" => Ok(Self::MaxTessControlOutputComponents),
            "max_tess_control_texture_image_units" => Ok(Self::MaxTessControlTextureImageUnits),
            "max_tess_control_uniform_components" => Ok(Self::MaxTessControlUniformComponents),
            "max_tess_control_total_output_components" => {
                Ok(Self::MaxTessControlTotalOutputComponents)
            }
            "max_tess_evaluation_input_components" => Ok(Self::MaxTessEvaluationInputComponents),
            "max_tess_evaluation_output_components" => Ok(Self::MaxTessEvaluationOutputComponents),
            "max_tess_evaluation_texture_image_units" => {
                Ok(Self::MaxTessEvaluationTextureImageUnits)
            }
            "max_tess_evaluation_uniform_components" => {
                Ok(Self::MaxTessEvaluationUniformComponents)
            }
            "max_tess_patch_components" => Ok(Self::MaxTessPatchComponents),
            "max_patch_vertices" => Ok(Self::MaxPatchVertices),
            "max_tess_gen_level" => Ok(Self::MaxTessGenLevel),
            "max_viewports" => Ok(Self::MaxViewports),
            "max_vertex_atomic_counters" => Ok(Self::MaxVertexAtomicCounters),
            "max_tess_control_atomic_counters" => Ok(Self::MaxTessControlAtomicCounters),
            "max_tess_evaluation_atomic_counters" => Ok(Self::MaxTessEvaluationAtomicCounters),
            "max_geometry_atomic_counters" => Ok(Self::MaxGeometryAtomicCounters),
            "max_fragment_atomic_counters" => Ok(Self::MaxFragmentAtomicCounters),
            "max_combined_atomic_counters" => Ok(Self::MaxCombinedAtomicCounters),
            "max_atomic_counter_bindings" => Ok(Self::MaxAtomicCounterBindings),
            "max_vertex_atomic_counter_buffers" => Ok(Self::MaxVertexAtomicCounterBuffers),
            "max_tess_control_atomic_counter_buffers" => {
                Ok(Self::MaxTessControlAtomicCounterBuffers)
            }
            "max_tess_evaluation_atomic_counter_buffers" => {
                Ok(Self::MaxTessEvaluationAtomicCounterBuffers)
            }
            "max_geometry_atomic_counter_buffers" => Ok(Self::MaxGeometryAtomicCounterBuffers),
            "max_fragment_atomic_counter_buffers" => Ok(Self::MaxFragmentAtomicCounterBuffers),
            "max_combined_atomic_counter_buffers" => Ok(Self::MaxCombinedAtomicCounterBuffers),
            "max_atomic_counter_buffer_size" => Ok(Self::MaxAtomicCounterBufferSize),
            "max_transform_feedback_buffers" => Ok(Self::MaxTransformFeedbackBuffers),
            "max_transform_feedback_interleaved_components" => {
                Ok(Self::MaxTransformFeedbackInterleavedComponents)
            }
            "max_cull_distances" => Ok(Self::MaxCullDistances),
            "max_combined_clip_and_cull_distances" => Ok(Self::MaxCombinedClipAndCullDistances),
            "max_samples" => Ok(Self::MaxSamples),
            "max_mesh_output_vertices_nv" => Ok(Self::MaxMeshOutputVerticesNv),
            "max_mesh_output_primitives_nv" => Ok(Self::MaxMeshOutputPrimitivesNv),
            "max_mesh_work_group_size_x_nv" => Ok(Self::MaxMeshWorkGroupSizeXNv),
            "max_mesh_work_group_size_y_nv" => Ok(Self::MaxMeshWorkGroupSizeYNv),
            "max_mesh_work_group_size_z_nv" => Ok(Self::MaxMeshWorkGroupSizeZNv),
            "max_task_work_group_size_x_nv" => Ok(Self::MaxTaskWorkGroupSizeXNv),
            "max_task_work_group_size_y_nv" => Ok(Self::MaxTaskWorkGroupSizeYNv),
            "max_task_work_group_size_z_nv" => Ok(Self::MaxTaskWorkGroupSizeZNv),
            "max_mesh_view_count_nv" => Ok(Self::MaxMeshViewCountNv),
            "max_mesh_output_vertices_ext" => Ok(Self::MaxMeshOutputVerticesExt),
            "max_mesh_output_primitives_ext" => Ok(Self::MaxMeshOutputPrimitivesExt),
            "max_mesh_work_group_size_x_ext" => Ok(Self::MaxMeshWorkGroupSizeXExt),
            "max_mesh_work_group_size_y_ext" => Ok(Self::MaxMeshWorkGroupSizeYExt),
            "max_mesh_work_group_size_z_ext" => Ok(Self::MaxMeshWorkGroupSizeZExt),
            "max_task_work_group_size_x_ext" => Ok(Self::MaxTaskWorkGroupSizeXExt),
            "max_task_work_group_size_y_ext" => Ok(Self::MaxTaskWorkGroupSizeYExt),
            "max_task_work_group_size_z_ext" => Ok(Self::MaxTaskWorkGroupSizeZExt),
            "max_mesh_view_count_ext" => Ok(Self::MaxMeshViewCountExt),
            "max_dual_source_draw_buffers_ext" => Ok(Self::MaxDualSourceDrawBuffersExt),
            _ => Err(format!("Invalid value: {}", s)),
        }
    }
}

impl FromPythonicString for shaderc::IncludeType {
    fn from_pythonic_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "relative" => Ok(Self::Relative),
            "standard" => Ok(Self::Standard),
            _ => Err(format!("Invalid value: {}", s)),
        }
    }
}

fn env_version_as_u32(env_version: shaderc::EnvVersion) -> u32 {
    use shaderc::EnvVersion::*;
    match env_version {
        Vulkan1_0 => 100,
        Vulkan1_1 => 101,
        Vulkan1_2 => 102,
        Vulkan1_3 => 103,
        OpenGL4_5 => 450,
        _ => 0,
    }
}

#[pyclass(unsendable)]
struct CompilationArtifact {
    inner: shaderc::CompilationArtifact,
}

#[pymethods]
impl CompilationArtifact {
    #[pyo3(signature = ())]
    fn len(&self) -> usize {
        self.inner.len()
    }

    #[pyo3(signature = ())]
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[pyo3(signature = ())]
    fn as_text(&self) -> PyResult<String> {
        Ok(self.inner.as_text())
    }

    #[pyo3(signature = ())]
    fn as_binary(&self) -> PyResult<Vec<u32>> {
        Ok(self.inner.as_binary().to_vec())
    }

    #[pyo3(signature = ())]
    fn as_binary_u8(&self) -> PyResult<Vec<u8>> {
        Ok(self.inner.as_binary_u8().to_vec())
    }

    #[pyo3(signature = ())]
    fn get_num_warnings(&self) -> u32 {
        self.inner.get_num_warnings()
    }

    #[pyo3(signature = ())]
    fn get_warning_messages(&self) -> String {
        self.inner.get_warning_messages()
    }
}

#[pyclass(unsendable)]
struct CompileOptions {
    inner: shaderc::CompileOptions<'static>,
}

#[pymethods]
impl CompileOptions {
    #[new]
    fn new() -> PyResult<Self> {
        match shaderc::CompileOptions::new() {
            Ok(opts) => Ok(CompileOptions { inner: opts }),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to create CompileOptions: {}",
                e
            ))),
        }
    }

    #[pyo3(signature = (env_str, version_str))]
    fn set_target_env(&mut self, env_str: &str, version_str: &str) -> PyResult<()> {
        let env = shaderc::TargetEnv::from_pythonic_string(env_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        let version = shaderc::EnvVersion::from_pythonic_string(version_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        self.inner.set_target_env(env, env_version_as_u32(version));
        Ok(())
    }

    #[pyo3(signature = (version_str))]
    fn set_target_spirv(&mut self, version_str: &str) -> PyResult<()> {
        let ver = shaderc::SpirvVersion::from_pythonic_string(version_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        self.inner.set_target_spirv(ver);
        Ok(())
    }

    #[pyo3(signature = (lang_str))]
    fn set_source_language(&mut self, lang_str: &str) -> PyResult<()> {
        let lang = shaderc::SourceLanguage::from_pythonic_string(lang_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        self.inner.set_source_language(lang);
        Ok(())
    }

    #[pyo3(signature = (version, profile_str))]
    fn set_forced_version_profile(&mut self, version: u32, profile_str: &str) -> PyResult<()> {
        let profile = shaderc::GlslProfile::from_pythonic_string(profile_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        self.inner.set_forced_version_profile(version, profile);
        Ok(())
    }

    #[pyo3(signature = (limit_str, value))]
    fn set_limit(&mut self, limit_str: &str, value: i32) -> PyResult<()> {
        let limit = shaderc::Limit::from_pythonic_string(limit_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        self.inner.set_limit(limit, value);
        Ok(())
    }

    #[pyo3(signature = (enable))]
    fn set_invert_y(&mut self, enable: bool) {
        self.inner.set_invert_y(enable);
    }

    #[pyo3(signature = (enable))]
    fn set_auto_bind_uniforms(&mut self, enable: bool) {
        self.inner.set_auto_bind_uniforms(enable);
    }

    #[pyo3(signature = (enable))]
    fn set_auto_combined_image_sampler(&mut self, enable: bool) {
        self.inner.set_auto_combined_image_sampler(enable);
    }

    #[pyo3(signature = (resource_kind_str, base))]
    fn set_binding_base(&mut self, resource_kind_str: &str, base: u32) -> PyResult<()> {
        let rk = shaderc::ResourceKind::from_pythonic_string(resource_kind_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        self.inner.set_binding_base(rk, base);
        Ok(())
    }

    #[pyo3(signature = (stage_str, resource_kind_str, base))]
    fn set_binding_base_for_stage(
        &mut self,
        stage_str: &str,
        resource_kind_str: &str,
        base: u32,
    ) -> PyResult<()> {
        let sk = shaderc::ShaderKind::from_pythonic_string(stage_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        let rk = shaderc::ResourceKind::from_pythonic_string(resource_kind_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        self.inner.set_binding_base_for_stage(sk, rk, base);
        Ok(())
    }

    #[pyo3(signature = (name, value=None))]
    fn add_macro_definition(&mut self, name: &str, value: Option<&str>) {
        self.inner.add_macro_definition(name, value);
    }

    #[pyo3(signature = (level_str))]
    fn set_optimization_level(&mut self, level_str: &str) -> PyResult<()> {
        let level = shaderc::OptimizationLevel::from_pythonic_string(level_str)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        self.inner.set_optimization_level(level);
        Ok(())
    }

    #[pyo3(signature = ())]
    fn set_generate_debug_info(&mut self) {
        self.inner.set_generate_debug_info();
    }

    #[pyo3(signature = ())]
    fn set_suppress_warnings(&mut self) {
        self.inner.set_suppress_warnings();
    }

    #[pyo3(signature = ())]
    fn set_warnings_as_errors(&mut self) {
        self.inner.set_warnings_as_errors();
    }

    #[pyo3(signature = (enable))]
    fn set_hlsl_functionality1(&mut self, enable: bool) {
        self.inner.set_hlsl_functionality1(enable);
    }
}

#[pyclass(unsendable)]
struct Compiler {
    inner: shaderc::Compiler,
}

#[pymethods]
impl Compiler {
    #[new]
    fn new() -> PyResult<Self> {
        match shaderc::Compiler::new() {
            Ok(c) => Ok(Compiler { inner: c }),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!(
                "Failed to create Compiler: {}",
                e
            ))),
        }
    }

    #[pyo3(signature = (source, stage, input_file_name="shader.glsl", entry_point="main", options=None))]
    fn compile_into_spirv(
        &self,
        source: &str,
        stage: &str,
        input_file_name: &str,
        entry_point: &str,
        options: Option<&CompileOptions>,
    ) -> PyResult<CompilationArtifact> {
        let shader_kind = shaderc::ShaderKind::from_pythonic_string(stage)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        let result = self
            .inner
            .compile_into_spirv(
                source,
                shader_kind,
                input_file_name,
                entry_point,
                options.map(|o| &o.inner),
            )
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(CompilationArtifact { inner: result })
    }

    #[pyo3(signature = (source, stage, input_file_name="shader.glsl", entry_point="main", options=None))]
    fn compile_into_spirv_assembly(
        &self,
        source: &str,
        stage: &str,
        input_file_name: &str,
        entry_point: &str,
        options: Option<&CompileOptions>,
    ) -> PyResult<CompilationArtifact> {
        let shader_kind = shaderc::ShaderKind::from_pythonic_string(stage)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;
        let result = self
            .inner
            .compile_into_spirv_assembly(
                source,
                shader_kind,
                input_file_name,
                entry_point,
                options.map(|o| &o.inner),
            )
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(CompilationArtifact { inner: result })
    }

    #[pyo3(signature = (source, input_file_name="shader.glsl", entry_point="main", options=None))]
    fn preprocess(
        &self,
        source: &str,
        input_file_name: &str,
        entry_point: &str,
        options: Option<&CompileOptions>,
    ) -> PyResult<CompilationArtifact> {
        let result = self
            .inner
            .preprocess(
                source,
                input_file_name,
                entry_point,
                options.map(|o| &o.inner),
            )
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(CompilationArtifact { inner: result })
    }

    #[pyo3(signature = (spirv_assembly, options=None))]
    fn assemble(
        &self,
        spirv_assembly: &str,
        options: Option<&CompileOptions>,
    ) -> PyResult<CompilationArtifact> {
        let result = self
            .inner
            .assemble(spirv_assembly, options.map(|o| &o.inner))
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(CompilationArtifact { inner: result })
    }
}

#[pyfunction]
fn get_spirv_version_py() -> (u32, u32) {
    shaderc::get_spirv_version()
}

#[pyfunction]
fn parse_version_profile_py(verprof: &str) -> Option<(u32, String)> {
    match shaderc::parse_version_profile(verprof) {
        Ok((v, p)) => Some((v, format!("{:?}", p))),
        Err(_) => None,
    }
}

#[pymodule]
fn shadercrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Compiler>()?;
    m.add_class::<CompileOptions>()?;
    m.add_class::<CompilationArtifact>()?;
    m.add_function(wrap_pyfunction!(get_spirv_version_py, m)?)?;
    m.add_function(wrap_pyfunction!(parse_version_profile_py, m)?)?;
    Ok(())
}
