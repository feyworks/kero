---@meta

---@class (exact) World: WorldMethods

---@class WorldClass: WorldMethods
local module = {}

---@class WorldMethods
local methods = {}

---Create a new world.
---@return World
---@nodiscard
function module.new() end

---Adds an entity to the world.
---@param self World
---@param ent Entity
function methods.add(self, ent) end

---Adds multiple entities to the world.
---@param self World
---@param ents Entity[]
function methods.add_many(self, ents) end

---Removes an entity from the world.
---@param self World
---@param ent Entity
function methods.remove(self, ent) end

---Remove all the provided entities from the world.
---@param self World
---@param ents Entity[]
function methods.remove_many(self, ents) end

---Removes all entities from the world.
---@param self World
function methods.clear(self) end

---Find a component of the requested type.
---@generic T: Component
---@param self World
---@param type `T`
---@return T?
---@nodiscard
function methods.find(self, type) end

---Find all components of the requested type.
---@generic T: Component
---@param self World
---@param type `T`
---@param fill T[]? If provided, this table will be filled and returned.
---@return T[]
---@nodiscard
function methods.find_all(self, type, fill) end

---Call a function on each component of the requested type.
---@generic T: Component
---@param self World
---@param type `T`
---@param fn fun(comp: T)
function methods.for_each(self, type, fn) end

---Iterate over the world's entities.
---@param self World
---@return fun(ent: World, idx: integer): integer, Entity
---@return World
---@return integer
function methods.iter(self) end

---Updates all components whose flags match any of the mask bits.
---@param self World
---@param mask integer?
function methods.update(self, mask) end

---Renders all components whose flags match any of the mask bits.
---@param self World
---@param mask integer?
function methods.render(self, mask) end

return module
