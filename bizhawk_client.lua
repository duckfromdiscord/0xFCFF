-- Settings (CHANGE THESE IF YOU WANT)
server = "http://localhost:8000/"

-- Local vars (DO NOT CHANGE)
addr = ""
last_rom_hash = ""

while true do
	emu.frameadvance();
	-- console.log(emu.framecount())
	if emu.framecount() % 100 == 0 then
		if last_rom_hash == "" then
			console.log("Checking new game")
			qs = "system="..emu.getsystemid().."&rom_hash="..gameinfo.getromhash().."&rom_name="..string.upper(gameinfo.getromname())
			-- console.log(qs)
			resp = comm.httpGet(server .. "game_biz?" .. qs)
			if resp == "None" then
				console.log("No addr for game")
			else
				addr = resp
				last_rom_hash = gameinfo.getromhash()
			end
		else
			qs = "system="..emu.getsystemid().."&id="..mainmemory.read_s16_le(tonumber(addr)).."&addr="..addr.."&rom_hash="..gameinfo.getromhash().."&rom_name="..string.upper(gameinfo.getromname())
			-- console.log(qs)
			comm.httpGet(server .. "submit_biz?" .. qs)
		end
	else
		-- .
	end
end
