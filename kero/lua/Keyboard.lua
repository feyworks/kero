---@meta

---@class Keyboard
local Keyboard = {}

---If the key is held down this frame.
---@param key Key
---@return boolean
---@nodiscard
function Keyboard.down(key) end

---If the key was pressed this frame.
---@param key Key
---@return boolean
---@nodiscard
function Keyboard.pressed(key) end

---If the key was released this frame.
---@param key Key
---@return boolean
---@nodiscard
function Keyboard.released(key) end

---If the key was repeated this frame.
---@param key Key
---@return boolean
---@nodiscard
function Keyboard.repeated(key) end

---If the key was pressed or repeated this frame.
---@param key Key
---@return boolean
---@nodiscard
function Keyboard.pressed_or_repeated(key) end

---Text that was typed by the keyboard this frame.
---@return string?
---@nodiscard
function Keyboard.text_input() end

---If left or right control is down.
---@return boolean
---@nodiscard
function Keyboard.ctrl() end

---If left or right shift is down.
---@return boolean
---@nodiscard
function Keyboard.shift() end

---If left or right alt is down.
---@return boolean
---@nodiscard
function Keyboard.alt() end

---If left or right command is down.
---@return boolean
---@nodiscard
function Keyboard.cmd() end

---If left or right control is down (command if the user is running on MacOS).
---@return boolean
---@nodiscard
function Keyboard.ctrl_or_cmd() end

return Keyboard