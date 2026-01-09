---@meta

---@alias ParamType
---     |"texture"
---     |"sampler"
---     |"int"
---     |"uint"
---     |"float"
---     |"vec2"
---     |"vec3"
---     |"vec4"
---     |"mat2"
---     |"mat3"
---     |"mat4"

---@class (exact) Shader: ShaderMethods

---@class ShaderClass: ShaderMethods
local module = {}

---@class ShaderMethods
local methods = {}

---Returns the default shader.
---@return Shader
---@nodiscard
function module.default() end

---Compile a shader from the source code.
---@param source string
---@return Shader
---@nodiscard
function module.new(source) end

---Return a table of all the shader's parameters.
---@param self Shader
---@return { [string]: ParamType }
---@nodiscard
function methods.params(self) end

---Return the type of the param (or `nil` if the param doesn't exist).
---@param self Shader
---@param name string
---@return ParamType?
---@nodiscard
function methods.param_type(self, name) end

return module