---@meta

---@class (exact) VertexBuffer: VertexBufferMethods

---@class VertexBufferClass: VertexBufferMethods
local module = {}

---@class VertexBufferMethods
local methods = {}

---Create a new vertex buffer with the provided capacity.
---@param capacity integer
---@return VertexBuffer
---@nodiscard
function module.new(capacity) end

---Create a new vertex buffer with the provided vertices.
---@param vertices Vertex[]
---@return VertexBuffer
---@nodiscard
function module.with(vertices) end

---How many vertices are in the buffer.
---@param self VertexBuffer
---@return integer
---@nodiscard
function methods.len(self) end

---Currently allocated vertex capacity the buffer has allocated.
---@param self VertexBuffer
---@return integer
---@nodiscard
function methods.capacity(self) end

---Set the buffer's vertices.
---@param self VertexBuffer
---@param vertices Vertex[]
function methods.upload(self, vertices) end

return module