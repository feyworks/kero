---@meta

---A rendering vertex.
---@class (exact) Vertex: VertexMethods
---@field pos Vec2
---@field tex Vec2
---@field col Color
---@field mode ColorMode

---@class VertexClass : VertexMethods
local module = {}

---@class VertexMethods
local methods = {}

---Create a new vertex.
---@param pos Vec2
---@param tex Vec2
---@param col Color
---@param mode ColorMode
---@return Vertex
---@nodiscard
function module.new(pos, tex, col, mode) end

---Create a new vertex.
---@param x number
---@param y number
---@param tx number
---@param ty number
---@param col Color
---@param mode ColorMode
---@return Vertex
---@nodiscard
function module.new(x, y, tx, ty, col, mode) end

---Create a new simple vertex.
---@param pos Vec2
---@param tex Vec2
---@return Vertex
---@nodiscard
function module.simple(pos, tex) end

---Create a new simple vertex.
---@param x number
---@param y number
---@param tx number
---@param ty number
---@return Vertex
---@nodiscard
function module.simple(x, y, tx, ty) end

---Create a new multiply vertex.
---@param pos Vec2
---@param tex Vec2
---@param col Color
---@return Vertex
---@nodiscard
function module.mult(pos, tex, col) end

---Create a new multiply vertex.
---@param x number
---@param y number
---@param tx number
---@param ty number
---@param col Color
---@return Vertex
---@nodiscard
function module.mult(x, y, tx, ty, col) end

---Create a new washout vertex.
---@param pos Vec2
---@param tex Vec2
---@param col Color
---@return Vertex
---@nodiscard
function module.wash(pos, tex, col) end

---Create a new washout vertex.
---@param x number
---@param y number
---@param tx number
---@param ty number
---@param col Color
---@return Vertex
---@nodiscard
function module.wash(x, y, tx, ty, col) end

---Create a new veto vertex.
---@param pos Vec2
---@param col Color
---@return Vertex
---@nodiscard
function module.veto(pos, col) end

---Create a new veto vertex.
---@param x number
---@param y number
---@param col Color
---@return Vertex
---@nodiscard
function module.veto(x, y, col) end

---Create a new misc vertex.
---@param pos Vec2
---@param tex Vec2
---@param col Color
---@return Vertex
---@nodiscard
function module.misc(pos, tex, col) end

---Create a new edge vertex.
---@param x number
---@param y number
---@param tx number
---@param ty number
---@param col Color
---@return Vertex
---@nodiscard
function module.edge(x, y, tx, ty, col) end

---Returns a temporary copy of this value.
---@param self Vertex
---@return Vertex
---@nodiscard
function methods.clone(self) end

---Boxes the value if not already boxed.
---@param self Vertex
---@return Vertex
---@nodiscard
function methods.box(self) end

---Boxes a copy of this value.
---@param self Vertex
---@return Vertex
---@nodiscard
function methods.box_clone(self) end

---Sets this vertex's position.
---@param self Vertex
---@param pos Vec2
function methods.set_pos(self, pos) end

---Sets this vertex's position.
---@param self Vertex
---@param x number
---@param y number
function methods.set_pos(self, x, y) end

---Sets this vertex's texture coord.
---@param self Vertex
---@param tex Vec2
function methods.set_tex(self, tex) end

---Sets this vertex's texture coord.
---@param self Vertex
---@param tx number
---@param ty number
function methods.set_tex(self, tx, ty) end

---Sets this vertex's color.
---@param self Vertex
---@param col Color
function methods.set_col(self, col) end

---Sets this vertex's mode.
---@param self Vertex
---@param mode ColorMode
function methods.set_mode(self, mode) end

return module