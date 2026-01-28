local Time = require "Time"

---@class (exact) Mover: MoverMethods

---@class MoverModule: MoverMethods
local module = {}

---@class MoverMethods: Component
local methods = {}

---Create a new mover.
---@param flags integer
---@return Mover
function module.new(flags)
    local obj = setmetatable({}, module) --[[@as Mover]]
    obj.flags = flags
    return obj
end

---@param self Mover
function methods.added(self)
    
end

---@param self Mover
function methods.removed(self)

end

---@param self Mover
function methods.spawned(self)
    
end

---@param self Mover
function methods.despawned(self)

end

---@param self Mover
function methods.update(self)
    local wave = Time.wave(-10, 10, 2.0) * Time.delta()
    self.entity:set_pos(self.entity.x + wave, self.entity.y - wave)
end

---@param self Mover
function methods.render(self)
    
end

local Component = require "Component"
Component.register("Mover", module, methods)

return module