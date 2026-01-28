---@meta

---@class (exact) Entity: EntityMethods
---@field active boolean
---@field visible boolean
---@field pos Vec2
---@field x number
---@field y number

---@class EntityClass: EntityMethods
local module = {}

---@class EntityMethods
local methods = {}

---Create a new entity.
---@return Entity
---@nodiscard
function module.new() end

---Create a new entity at the specified position.
---@param pos Vec2
---@return Entity
---@nodiscard
function module.new_at(pos) end

---Create a new entity at the specified position.
---@param x number
---@param y number
---@return Entity
---@nodiscard
function module.new_at(x, y) end

---World the entity is in.
---@param self Entity
---@return World?
---@nodiscard
function methods.world(self) end

---Remove this entity from the world, if it's in one.
---@param self Entity
function methods.remove_self(self) end

---@param self Entity
---@param val Vec2
function methods.set_pos(self, val) end

---@param self Entity
---@param x number
---@param y number
function methods.set_pos(self, x, y) end

---@param self Entity
---@return number
---@nodiscard
function methods.set_x(self, val) end

---@param self Entity
---@return number
---@nodiscard
function methods.set_y(self, val) end

---Moves the entity by the provided amount.
---@param self Entity
---@param amount Vec2
function methods.move_by(self, amount) end

---Moves the entity by the provided amount.
---@param self Entity
---@param x number
---@param y number
function methods.move_by(self, x, y) end

---Add a component.
---@generic T: Component
---@param self Entity
---@param comp T
---@return T
function methods.add(self, comp) end

---Remove a component.
---@param self Entity
---@param comp Component|string
function methods.remove(self, comp) end

---Remove all components of the type.
---@generic T: Component
---@param self Entity
---@param type `T`
function methods.remove_all(self, type) end

---Removes all components.
---@param self Entity
function methods.clear(self) end

---Get a component of the type.
---@generic T: Component
---@param self Entity
---@param type `T`
---@return T?
---@nodiscard
function methods.get(self, type) end

---Call the function on each component of the type.
---@generic T: Component
---@param self Entity
---@param type `T`
---@param call fun(comp: T)
function methods.for_each(self, type, call) end

---Iterate over the entity's components.
---@param self Entity
---@return fun(ent: Entity, idx: integer): integer, Component
---@return Entity
---@return integer
function methods.iter(self) end

---Iterate over the entity's components of the provided type.
---@generic T: Component
---@param self Entity
---@param type `T`
---@return fun(ent: Entity, idx: integer): integer, T
---@return Entity
---@return integer
function methods.iter_type(self, type) end

return module