Medal's LuaU decompiler

All credits to this project goes to in honor and memory of:
Jujhar Singh (KowalskiFX)
Mathias Pedersen (Costomality)

While details of how they passed and our relationship with them are completely irrelevant its better if their legacy 
does not go in vain. 

Keep the Singh and Pedersen family in you guys prayers.
We love you both.

## Script

```lua
getgenv().decompile = function(script_instance)
    local bytecode = getscriptbytecode(script_instance)
    local encoded = crypt.base64.encode(bytecode)
    return request(
        {
            Url = "http://localhost:3000/decompile",
            Method = "POST",
            Body = encoded
        }
    ).Body
end

local synsaveinstance = loadstring(game:HttpGet("https://raw.githubusercontent.com/luau/SynSaveInstance/main/saveinstance.luau"))()
local Options = {
  SafeMode = true,
  ShutdownWhenDone = true,
  mode = "scripts",
  NilInstances = true,
}
synsaveinstance(Options)
```
