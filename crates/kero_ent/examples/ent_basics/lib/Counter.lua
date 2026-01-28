---@meta

---@class (exact) Counter: CounterMethods

---@class CounterModule: CounterMethods
local module = {}

---@class CounterMethods: Component
local methods = {}

---Create a new counter component.
---@param flags integer
---@return Counter
---@nodiscard
function module.new(flags) end

---The counter's current value.
---@param self Counter
---@return integer
---@nodiscard
function methods.count(self) end

---Reset the counter to zero.
---@param self Counter
function methods.reset(self) end

return module