const rustLogo = createRustLogo()

insertIntoDom(rustLogo)

function createRustLogo() {
  const div = document.createElement('div')

  div.classList.add('background-tile')

  div.style.width = '100vw'
  div.style.height = '100vh'
  div.style.zIndex = '1'
  div.style.top = '1'
  div.style.left = '1'

  return div
}

function insertIntoDom(div) {
  const parent = document.querySelector('.rust-tutorial')

  parent.append(div)
}