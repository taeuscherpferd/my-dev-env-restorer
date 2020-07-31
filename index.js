const fs = require('fs');
const path = require('path')
const http = require('http');
const https = require('https');
const opn = require('opn')
const pfs = fs.promises;

const DOWNLOAD_DIR = path.join(process.env.HOME || process.env.USERPROFILE, 'Downloads/');

const readURLs = async () => {
  let conts = await pfs.readFile('./Windows/programs', 'utf8')
  let relativeConts = conts.split('------------')[0]

  return relativeConts.split('\n')
}

/**
 * Downloads file from remote HTTP[S] host and puts its contents to the
 * specified location.
 */
async function download(url, filePath) {
  const proto = !url.charAt(4).localeCompare('s') ? https : http;

  return new Promise((resolve, reject) => {
    let file = fs.createWriteStream(filePath);
    let fileInfo = null;

    const request = proto.get(url, response => {
      if (response.statusCode !== 200) {
        reject(new Error(`Failed to get '${url}' (${response.statusCode})`));
        return;
      }

      fileInfo = {
        mime: response.headers['content-type'],
        size: parseInt(response.headers['content-length'], 10),
      };

      response.pipe(file);
    });

    // The destination stream is ended by the time it's called
    file.on('finish', () => resolve(fileInfo));

    request.on('error', err => {
      fs.unlink(filePath, () => reject(err));
    });

    file.on('error', err => {
      fs.unlink(filePath, () => reject(err));
    });

    request.end();
  });
}

const main = async () => {
  let urlArray
  try {
    urlArray = await readURLs()
  }
  catch (err) {
    console.log(err)
  }

  for (const progUrl of urlArray) {
    try {
      opn(progUrl)
    }
    catch (err) {
      console.log(err)
    }
  }
}

main()


