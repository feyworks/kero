---@meta

---@class (exact) Instant: InstantMethods

---@class InstantClass: InstantMethods
local module = {}

---@class InstantMethods
local methods = {}

---Returns an instant corresponding to now.
---@return Instant
---@nodiscard
function module.now() end

---Time elapsed since this instant in seconds.
---@param self Instant
---@return number
---@nodiscard
function methods.elapsed_secs(self) end

---Time elapsed since this instant in milliseconds.
---@param self Instant
---@return integer
---@nodiscard
function methods.elapsed_millis(self) end

---Time elapsed since this instant in microseconds.
---@param self Instant
---@return integer
---@nodiscard
function methods.elapsed_micros(self) end

---Time elapsed since this instant in nanoseconds.
---@param self Instant
---@return integer
---@nodiscard
function methods.elapsed_nanos(self) end

return module