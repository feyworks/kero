---@meta

---@class Component: ComponentMethods
---@field active boolean
---@field visible boolean
---@field flags integer
---@field depth number
---@field entity Entity?
---@field added fun(self: Component)?
---@field removed fun(self: Component)?
---@field spawned fun(self: Component)?
---@field despawned fun(self: Component)?
---@field update fun(self: Component)?
---@field render fun(self: Component)?

---@class ComponentClass: ComponentMethods
local module = {}

---Register a component type.
---@param name string
---@param class table
---@param methods table
function module.register(name, class, methods) end

---@class ComponentMethods
local methods = {}

---Returns the component's type name.
---@param self Component
---@return string
---@nodiscard
function methods.type_name(self) end

---Returns the component's world.
---@param self Component
---@return World?
---@nodiscard
function methods.world(self) end

---Get a component from the entity, if one exists.
---@generic T: Component
---@param self Component
---@param type `T`
---@return T?
---@nodiscard
function methods.get(self, type) end

---Remove this component from its entity.
---@param self Component
function methods.remove_self(self) end

-- ---Called when the component is added to an entity.
-- ---@param self Component
-- function methods.added(self) end

-- ---Called when the component is removed from an entity.
-- ---@param self Component
-- function methods.removed(self) end

-- ---Called when the component is added to a world.
-- ---@param self Component
-- function methods.spawned(self) end

-- ---Called when the component is removed from a world.
-- ---@param self Component
-- function methods.despawned(self) end

-- ---Called when the component updates.
-- ---@param self Component
-- function methods.update(self) end

-- ---Called when the component renders.
-- ---@param self Component
-- function methods.render(self) end

return module