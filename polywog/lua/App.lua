---@meta

---@class AppModule
local App = {}

---Quit the app.
function App.quit() end

---If `quit()` was called and the app is scheduled to shutdown.
function App.quit_requested() end

---Restart the app, which will reload all Lua modules and reset from `Main.lua`.
function App.restart() end

---If `restart()` was called and the app is scheduled to restart.
function App.restart_requested() end

---Path to the game's cache directory.
---
--- |Platform | Example                                              |
--- | ------- | ---------------------------------------------------- |
--- | Linux   | /home/alice/.cache/appname                           |
--- | macOS   | /Users/Alice/Library/Caches/Org-Name.App-Name        |
--- | Windows | C:\Users\Alice\AppData\Local\Org Name\App Name\cache |
---@return string
function App.cache_dir() end

---Path to the game's config directory.
---
--- |Platform | Example                                                    |
--- | ------- | ---------------------------------------------------------- |
--- | Linux   | /home/alice/.config/appname                                |
--- | macOS   | /Users/Alice/Library/Application Support/Org-Name.App-Name |
--- | Windows | C:\Users\Alice\AppData\Roaming\Org Name\App Name\config    |
---@return string
function App.config_dir() end

---Path to the game's local config directory.
---
--- |Platform | Example                                                    |
--- | ------- | ---------------------------------------------------------- |
--- | Linux   | /home/alice/.config/appname                                |
--- | macOS   | /Users/Alice/Library/Application Support/Org-Name.App-Name |
--- | Windows | C:\Users\Alice\AppData\Local\Org Name\App Name\config      |
---@return string
function App.config_local_dir() end

---Path to the game's data directory.
---
--- |Platform | Example                                                         |
--- | ------- | --------------------------------------------------------------- |
--- | Linux   | /home/alice/.local/share/appname                                |
--- | macOS   | /Users/Alice/Library/Application Support/Org-Name.App-Name      |
--- | Windows | Windows | C:\Users\Alice\AppData\Roaming\Org Name\App Name\data |
---@return string
function App.data_dir() end

---Path to the game's local data directory.
---
--- |Platform | Example                                                    |
--- | ------- | ---------------------------------------------------------- |
--- | Linux   | /home/alice/.local/share/appname                           |
--- | macOS   | /Users/Alice/Library/Application Support/Org-Name.App-Name |
--- | Windows | C:\Users\Alice\AppData\Local\Org Name\App Name\data        |
---@return string
function App.data_local_dir() end

---Path to the game's local preference directory.
---
--- |Platform | Example                                                 |
--- | ------- | ------------------------------------------------------- |
--- | Linux   | /home/alice/.config/appname                             |
--- | macOS   | /Users/Alice/Library/Preferences/Org-Name.App-Name      |
--- | Windows | C:\Users\Alice\AppData\Roaming\Org Name\App Name\config |
---@return string
function App.preferences_dir() end

return App