    const chatDiv = document.querySelector('.chat')
    
    // Create WebSocket connection.
    const socket = new WebSocket('ws://localhost:3002');

    // Connection opened
    socket.addEventListener('open', function (event) {
        socket.send(JSON.stringify({message: "front end starting up"}));
    });

    // Listen for messages
    socket.addEventListener('message', function (event) {
        const message = JSON.parse(event.data);
        console.log('Message from server ', message);
        generateTag(message)
          .then(injectIntoDom)
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
    }