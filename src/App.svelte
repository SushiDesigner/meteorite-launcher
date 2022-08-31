<main>
	<style src="./app.css"></style>
 <video class="embed-responsive-item" muted autoplay loop src="http://mete0r.xyz/assets/videos/logo.webm" type="video/webm" style="margin: 0 auto; max-width: 30rem; max-height: 30rem;"></video>
    <div class="flex justify-center items-center text-white">
    <h2 id="text">Please Wait.</h2>
  </div>

    <div class="flex justify-center items-center">
      <progress class="progress w-70 progress-primary"></progress>
    </div>
</main>

<script lang="ts">
		 import { getMatches } from '@tauri-apps/api/cli';
	 import { message } from '@tauri-apps/api/dialog';
	 import { invoke } from '@tauri-apps/api/tauri';
	 import { exit } from '@tauri-apps/api/process';
     import { dataDir } from '@tauri-apps/api/path';
	 import { fetch } from '@tauri-apps/api/http';
	 const pub = `-----BEGIN PUBLIC KEY-----
MIGeMA0GCSqGSIb3DQEBAQUAA4GMADCBiAKBgHggU4nE398eHvM8ZLVLkTBuScf5
rIpqeeBdvwErxEYsAdrn55tgwN9t09BMJPapngGrbq+7K0g0nxyolZuBKwsv+8OE
5ZIYC8k6t282bLYelDKmwHU+tfnMS80x5I/XIJXBkAtj2ZYgk0dCtqtsHZlyBUu2
9wASD48oCaaEnre7AgMBAAE=
-----END PUBLIC KEY-----`
	async function main() {
	 const matches = await getMatches();
       if (matches.args.first.value === false){
        // launched without args
        message("Join on website please.", "Info");
        exit(1);
       }
       var url_to_open = matches.args.first.value[1]
       url_to_open = url_to_open.replace("meteorite-launch://","")
      url_to_open = url_to_open.replace('/',"")
      url_to_open = url_to_open.split('[') // this will turn it into a array so we can have auth and placeid different
	  let dataDirPath = await dataDir();
	  const exists = await invoke('exists', { path: dataDirPath+'MeteoritePlayer/RobloxPlayerBetaRaw.exe'});
	  if (exists === true){
		// meteorite already installed
		const hash = await fetch('http://mete0r.xyz/assets/hash.txt', {
  			method: 'GET',
 			 timeout: 30,
			 responseType: 2
			});
	const currenthash = await invoke('hash', {path: dataDirPath+'MeteoritePlayer/RobloxPlayerBetaRaw.exe'});
	if (hash.data === currenthash){
		//updated
		//launch
		// rust doesn't like \ for some reason so quick hack
		const sandboxieexists =  await invoke('exists', { path: "C:/Program Files/Sandboxie-Plus/Start.exe"});
		if (sandboxieexists === true){
			await invoke('exec', {exe:"C:/\"Program Files\"/Sandboxie-Plus/Start.exe "+dataDirPath.replace(/\\/g, '/')+"MeteoritePlayer/RobloxPlayerBetaRaw.exe -a \"mete0r.xyz/login/Negotiate\" -j \"http://mete0r.xyz/game/placelauncher?name="+url_to_open[0]+"&auth="+url_to_open[1]+"\" -t \"ticket\""});
			exit(1);
		}
		await invoke('exec', {exe:dataDirPath.replace(/\\/g, '/')+"MeteoritePlayer/RobloxPlayerBetaRaw.exe -a \"mete0r.xyz/login/Negotiate\" -j \"http://mete0r.xyz/game/placelauncher?name="+url_to_open[0]+"&auth="+url_to_open[1]+"\" -t \"ticket\""});
		exit(1);
	}



	  }
	  // download and extract if it isn't already installed or update
	  document.getElementById('text').innerHTML = "Downloading"
      await invoke('download_file', { url: 'http://mete0r.xyz/assets/release.zip',path: dataDirPath+'release.zip'});
	  document.getElementById('text').innerHTML = "Extracting"
	  await invoke('extract', { path: dataDirPath+'release.zip',extracto: dataDirPath+'MeteoritePlayer'});
	  document.getElementById('text').innerHTML = "Done!"
	  const releaseexists = await invoke('exists', { path: dataDirPath+'release.zip'});
	  if (releaseexists === true){
		await invoke('removefile', {file:dataDirPath+'release.zip'});
	  }
	  const sandboxieexists =  await invoke('exists', { path: "C:/Program Files/Sandboxie-Plus/Start.exe"});
		if (sandboxieexists === true){
			await invoke('exec', {exe:"C:/\"Program Files\"/Sandboxie-Plus/Start.exe "+dataDirPath.replace(/\\/g, '/')+"MeteoritePlayer/RobloxPlayerBetaRaw.exe -a \"mete0r.xyz/login/Negotiate\" -j \"http://mete0r.xyz/game/placelauncher?name="+url_to_open[0]+"&auth="+url_to_open[1]+"\" -t \"ticket\""});
			exit(1);
		}
	  await invoke('exec', {exe:dataDirPath.replace(/\\/g, '/')+"MeteoritePlayer/RobloxPlayerBetaRaw.exe -a \"mete0r.xyz/login/Negotiate\" -j \"http://mete0r.xyz/game/placelauncher?name="+url_to_open[0]+"&auth="+url_to_open[1]+"\" -t \"ticket\""});
	  exit(1);
}
main()
</script>

