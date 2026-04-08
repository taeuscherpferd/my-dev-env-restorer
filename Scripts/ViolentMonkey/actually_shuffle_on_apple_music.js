// ==UserScript==
// @name        New script apple.com
// @namespace   Violentmonkey Scripts
// @match       https://music.apple.com/us/library/songs*
// @grant       none
// @version     1.0
// @author      -
// @description 12/12/2024, 6:53:50 AM
// ==/UserScript==

const delay = ms => new Promise(res => setTimeout(res, ms));

const getContentContainer = () => {
 return document.querySelector("div[data-testid='content-container']")
}
const getSongs = () => {
  return document.querySelector("div[data-testid='virtual-rows']").children
}
const getMusicControls = async () => {
  let retryCount = 0;
  let ampChromePlayer = null;
  let musicControls = null;

  while (retryCount < 10) {
    retryCount++;
    ampChromePlayer = document.querySelector("amp-chrome-player")
    console.log("wat")
    if (ampChromePlayer !== null) {
      musicControls = ampChromePlayer?.shadowRoot?.querySelector("apple-music-playback-controls")?.shadowRoot?.querySelector('.music-controls') ?? null
      if (musicControls) return musicControls
    }
    await delay(1000)
  }
  return null
}

async function expandAndShuffle() {
  const contentContainer = getContentContainer()
  contentContainer.style.minHeight = '50000px'

  await delay(2000);
  
  const songs = getSongs()
  const numberOfSongs = songs.length

  const randomSongIndex = Math.floor(Math.random() * numberOfSongs)
  const randomSong = songs[randomSongIndex]

  randomSong.querySelector("button[aria-label='Play']").click()
  const musicControls = await getMusicControls()
  musicControls.shadowRoot.querySelector("button").click()
}

const createTrueShuffleButton = () => {
  const shuffleButton = document.createElement('button')
  shuffleButton.innerText = "Actually Shuffle"
  shuffleButton.onclick = expandAndShuffle
  return shuffleButton
}


async function main() {
  const shuffleButton = createTrueShuffleButton();
  const musicControls = await getMusicControls()
  if (musicControls) {
    musicControls.appendChild(shuffleButton)
  }
};

main()

