---@meta

---A video mode available to a monitor.
---@class (exact) VideoMode: VideoModeMethods

---@class VideoModeClass: VideoModeMethods
local module = {}

---@class VideoModeMethods
local methods = {}

---Resolution of the video mode.
---@param self VideoMode
---@return integer
---@return integer
---@nodiscard
function methods.size(self) end

---Horizontal resolution of the video mode.
---@param self VideoMode
---@return integer
---@nodiscard
function methods.width(self) end

---Vertical resolution of the video mode.
---@param self VideoMode
---@return integer
---@nodiscard
function methods.height(self) end

---Bit depth of the video mode (how many bits per color).
---@param self VideoMode
---@return integer
---@nodiscard
function methods.bit_depth(self) end

---Refresh rate of the video mode in `mHz`.
---@param self VideoMode
---@return integer
---@nodiscard
function methods.refresh_rate(self) end

---The monitor this video mode belongs to.
---@param self VideoMode
---@return Monitor
---@nodiscard
function methods.monitor(self) end

return module