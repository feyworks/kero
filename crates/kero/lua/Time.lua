---@meta

---@class Time
local Time = {}

---FPS the app is running at.
---@return number
---@nodiscard
function Time.fps() end

---Delta time since the last frame.
---@return number
---@nodiscard
function Time.delta() end

---Total time passed since the app started.
---@return number
---@nodiscard
function Time.since_startup() end

---Current frame number.
---@return integer
---@nodiscard
function Time.frame() end

---Flicker between true and false.
---@param on_time number
---@param off_time number?
---@return boolean
---@nodiscard
function Time.flicker(on_time, off_time) end

---@param from number
---@param to number
---@param duration number
---@param offset_percent number?
---@return number
---@nodiscard
function Time.wave(from, to, duration, offset_percent) end

return Time