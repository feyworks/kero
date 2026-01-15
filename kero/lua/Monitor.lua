---@meta

---A connected monitor.
---@class (exact) Monitor: MonitorMethods

---@class MonitorClass: MonitorMethods
local module = {}

---@class MonitorMethods
local methods = {}

---Returns a list of all available monitors.
---@return Monitor[]
---@nodiscard
function module.all() end

---Returns the primary monitor.
---@return Monitor?
---@nodiscard
function module.primary() end

---Name of the monitor
---@param self Monitor
---@return string?
---@nodiscard
function methods.name(self) end

---The monitor's dpi-independent size.
---@param self Monitor
---@return integer
---@return integer
---@nodiscard
function methods.size(self) end

---The monitor's dpi-independent width.
---@param self Monitor
---@return integer
---@nodiscard
function methods.width(self) end

---The monitor's dpi-independent height.
---@param self Monitor
---@return integer
---@nodiscard
function methods.height(self) end

---Size of the monitor in pixels.
---@param self Monitor
---@return integer
---@return integer
---@nodiscard
function methods.pixel_size(self) end

---Width of the monitor in pixels.
---@param self Monitor
---@return integer
---@nodiscard
function methods.pixel_width(self) end

---Height of the monitor in pixels.
---@param self Monitor
---@return integer
---@nodiscard
function methods.pixel_height(self) end

---The monitor's dpi-independent position.
---@param self Monitor
---@return integer
---@return integer
---@nodiscard
function methods.pos(self) end

---The monitor's dpi-independent x-position.
---@param self Monitor
---@return integer
---@nodiscard
function methods.x(self) end

---The monitor's dpi-independent y-position.
---@param self Monitor
---@return integer
---@nodiscard
function methods.y(self) end

---Pixel position of the monitor.
---@param self Monitor
---@return integer
---@return integer
---@nodiscard
function methods.pixel_pos(self) end

---Pixel x-position of the monitor.
---@param self Monitor
---@return integer
---@nodiscard
function methods.pixel_x(self) end

---Pixel y-position of the monitor.
---@param self Monitor
---@return integer
---@nodiscard
function methods.pixel_y(self) end

---Refresh rate of the monitor in `mHz`.
---@param self Monitor
---@return integer?
---@nodiscard
function methods.refresh_rate(self) end

---Scale factor of the monitor.
---@param self Monitor
---@return number
---@nodiscard
function methods.scale_factor(self) end

---Fullscreen video modes available to this monitor.
---@param self Monitor
---@return VideoMode[]
---@nodiscard
function methods.video_modes(self) end

return module