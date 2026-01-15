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
function Time.total() end

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

return Time