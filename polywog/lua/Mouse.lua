---@meta

---@class Mouse
local Mouse = {}

---Position of the mouse.
---@return Vec2
---@nodiscard
function Mouse.pos() end

---X position of the mouse.
---@return number
---@nodiscard
function Mouse.x() end

---Y position of the mouse.
---@return number
---@nodiscard
function Mouse.y() end

---Vertical scroll of the mouse.
---@return number
---@nodiscard
function Mouse.scroll() end

---If the button is held down this frame.
---@param btn MouseButton
---@return boolean
---@nodiscard
function Mouse.down(btn) end

---If the button was pressed this frame.
---@param btn MouseButton
---@return boolean
---@nodiscard
function Mouse.pressed(btn) end

---If the button was released this frame.
---@param btn MouseButton
---@return boolean
---@nodiscard
function Mouse.released(btn) end

return Mouse