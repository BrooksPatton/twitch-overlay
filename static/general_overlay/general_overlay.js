    const chatDiv = document.querySelector('.chat')
    
    // Create WebSocket connection.
    const socket = new WebSocket('ws://localhost:3002');

    // Listen for messages
    socket.addEventListener('message', function (event) {
        const message = JSON.parse(event.data);
        console.log('Message from server ', message);
        generateTag(message)
          .then(injectIntoDom)
          .then(fadeNodeIn)
          .then(delay)
          .then(fadeNodeOut)
          .then(addEndingEventListener)
    });

    function generateTag({nickname, message}) {
      const container = document.createElement('div')
      const header = document.createElement('header')
      const text = document.createElement('p')

      header.textContent = nickname
      text.textContent = message
      container.appendChild(header)
      container.appendChild(text)
      
      return Promise.resolve(container)
    }

    function injectIntoDom(tag) {
      chatDiv.appendChild(tag)

      return tag
    }

    function fadeNodeIn(tag) {
      tag.classList.add('fade-in')

      return tag
    }

    function delay(tag) {
      return new Promise(resolve => {
        setTimeout(() => {
          resolve(tag)
        }, 10000)
      })
    }

    function fadeNodeOut(tag) {
      tag.classList.add('fade-out')

      return tag
    }

    function addEndingEventListener(tag) {
      tag.addEventListener('animationend', function(event) {
        tag.remove()
      })
    }