---@meta

---A globally unique identifier.
---@class Guid: GuidMethods

---@class GuidClass: GuidMethods
---@overload fun(): Guid
local module = {}

---@class GuidMethods
local methods = {}

---Generator a new ID.
---@return Guid
---@nodiscard
function module.new() end

return module