const fs = require('fs')

fs.writeFileSync(
	'install-linux.sh',
	String(fs.readFileSync('release/install-linux.sh'))
		.replaceAll('&&DATA&&', fs.readFileSync('release/cesilc').toString('base64'))
		.replaceAll('&&VERSION&&', String(fs.readFileSync('src/assets/version.txt')).trim())
)

fs.chmodSync('install-linux.sh', '755')
