---@meta

---@alias GamepadId integer
---@alias GamepadStatus "wired"|"draining"|"charging"|"charged"

---@class Gamepad
local Gamepad = {}

---All connected gamepads.
---@param fill GamepadId[]?
---@return GamepadId[]?
---@nodiscard
function Gamepad.all(fill) end

---All gamepads connected this frame.
---@param fill GamepadId[]?
---@return GamepadId[]?
---@nodiscard
function Gamepad.newly_connected(fill) end

---The last active gamepad.
---@return GamepadId?
---@nodiscard
function Gamepad.last_active() end

---How many gamepads are connected.
---@return integer
---@nodiscard
function Gamepad.count() end

---Name of the gamepad.
---@param id GamepadId?
---@return string?
---@nodiscard
function Gamepad.name(id) end

---If the gamepad is connected.
---@param id GamepadId
---@return boolean
---@nodiscard
function Gamepad.connected(id) end

---If the gamepad was connected this frame.
---@param id GamepadId?
---@return boolean
---@nodiscard
function Gamepad.was_connected(id) end

---Status of the gamepad. For "charging" and "draining", a charge percent from 0-100 will be returned.
---@param id GamepadId?
---@return GamepadStatus?
---@return number?
---@nodiscard
function Gamepad.status(id) end

---If the button is held down.
---@param btn GamepadButton
---@param id GamepadId?
---@return boolean
---@nodiscard
function Gamepad.down(btn, id) end

---If the button was pressed this frame.
---@param btn GamepadButton
---@param id GamepadId?
---@return boolean
---@nodiscard
function Gamepad.pressed(btn, id) end

---If the button was releaseds this frame.
---@param btn GamepadButton
---@param id GamepadId?
---@return boolean
---@nodiscard
function Gamepad.released(btn, id) end

---If the button was repeated this frame.
---@param btn GamepadButton
---@param id GamepadId?
---@return boolean
---@nodiscard
function Gamepad.repeated(btn, id) end

---Value of the axis.
---@param axis GamepadAxis
---@param id GamepadId?
---@return boolean
---@nodiscard
function Gamepad.axis(axis, id) end

---If the axis was changed this frame.
---@param axis GamepadAxis
---@param id GamepadId?
---@return boolean
---@nodiscard
function Gamepad.axis_changed(axis, id) end

return Gamepad