---@meta

---@class (exact) IndexBuffer: IndexBufferMethods

---@class IndexBufferClass: IndexBufferMethods
local module = {}

---@class IndexBufferMethods
local methods = {}

---Create a new index buffer with the provided capacity.
---@param capacity integer
---@return IndexBuffer
---@nodiscard
function module.new(capacity) end

---Create a new index buffer with the provided indices.
---@param indices integer[]
---@return IndexBuffer
---@nodiscard
function module.with(indices) end

---How many indices are in the buffer.
---@param self IndexBuffer
---@return integer
---@nodiscard
function methods.len(self) end

---Currently allocated index capacity the buffer has allocated.
---@param self IndexBuffer
---@return integer
---@nodiscard
function methods.capacity(self) end

---Set the buffer's indices.
---@param self IndexBuffer
---@param indices integer[]
function methods.upload(self, indices) end

return module