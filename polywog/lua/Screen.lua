---@meta

---@class (exact) Screen: ScreenMethods

---@class ScreenClass : ScreenMethods
local module = {}

---@class ScreenMethods
local methods = {}

---Create a new screen of the provided size. It will always keep this aspect ratio,
---but its size will scale incrementally to fill as much window space as possible.
---@param w integer
---@param h integer
---@param fractional boolean
---@return Screen
---@nodiscard
function module.new_frame(w, h, fractional) end

---Create a new screen that will always have the provided pixel scale, and its size
---will be calculated to fit as much of the window as possible.
---@param scale number
---@return Screen
---@nodiscard
function module.new_fill(scale) end

---The screen's render surface.
---@param self Screen
---@return Surface
---@nodiscard
function methods.surface(self) end

---The screen's size.
---@param self Screen
---@return Vec2
---@nodiscard
function methods.size(self) end

---The screen's width.
---@param self Screen
---@return integer
---@nodiscard
function methods.width(self) end

---The screen's height.
---@param self Screen
---@return integer
---@nodiscard
function methods.height(self) end

---Where the screen will draw on the window.
---@param self Screen
---@return Rect
---@nodiscard
function methods.window_rect(self) end

---The screen's pixel scale.
---@param self Screen
---@return number
---@nodiscard
function methods.scale(self) end

---Mouse position in screen space.
---@param self Screen
---@return Vec2
---@nodiscard
function methods.mouse_pos(self) end

---Mouse x-position in screen space.
---@param self Screen
---@return number
---@nodiscard
function methods.mouse_x(self) end

---Mouse y-position in screen space.
---@param self Screen
---@return number
---@nodiscard
function methods.mouse_y(self) end

---Update the screen surface and mouse position. Should be called at the
---beginning of every frame the screen will be used.
---@param self Screen
function methods.update(self) end

---Maps the position from window to screen space.
---@param self Screen
---@param pos Vec2
---@return Vec2
---@nodiscard
function methods.map_pos(self, pos) end

---Maps the position from window to screen space.
---@param self Screen
---@param x number
---@param y number
---@return number
---@return number
---@nodiscard
function methods.map_pos(self, x, y) end

---Make this screen's surface the render target.
---@param self Screen
---@param draw Draw
---@param clear_color Color?
function methods.set_as_draw_surface(self, draw, clear_color) end

---Draws the screen to the window.
---@param self Screen
---@param draw Draw
---@param clear_color Color?
function methods.draw_to_window(self, draw, clear_color) end

return module