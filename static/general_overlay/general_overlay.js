    const chatDiv = document.querySelector('.chat')
    
    // Create WebSocket connection.
    var chatClient = function chatClient(options){
        this.username = options.username;
        this.password = options.password;
        this.channel = options.channel;
    
        this.server = 'irc-ws.chat.twitch.tv';
        this.port = 443;
    }
    
    chatClient.prototype.open = function open(){
        this.webSocket = new WebSocket('wss://' + this.server + ':' + this.port + '/', 'irc');
    
        this.webSocket.onmessage = this.onMessage.bind(this);
        this.webSocket.onerror = this.onError.bind(this);
        this.webSocket.onclose = this.onClose.bind(this);
        this.webSocket.onopen = this.onOpen.bind(this);
    };
    
    chatClient.prototype.onError = function onError(message){
        console.log('Error: ' + message);
    };
    
  
    chatClient.prototype.onMessage = function onMessage(message){
        if(message !== null){
            var parsed = this.parseMessage(message.data);
            if(parsed !== null){
                if(parsed.command === "PRIVMSG") {
                    generateTag(parsed)
                        .then(injectIntoDom)
                        .then(fadeNodeIn)
                        .then(delay)
                        .then(fadeNodeOut)
                        .then(addEndingEventListener)
                } else if(parsed.command === "PING") {
                    this.webSocket.send("PONG :" + parsed.message);
                }
            }
        }
    };
    
    chatClient.prototype.onOpen = function onOpen(){
        var socket = this.webSocket;
    
        if (socket !== null && socket.readyState === 1) {
            console.log('Connecting and authenticating...');
    
            socket.send('CAP REQ :twitch.tv/tags twitch.tv/commands twitch.tv/membership');
            socket.send('PASS ' + this.password);
            socket.send('NICK ' + this.username);
            socket.send('JOIN ' + this.channel);
        }
    };
    
    chatClient.prototype.onClose = function onClose(){
        console.log('Disconnected from the chat server.');
    };
    
    chatClient.prototype.close = function close(){
        if(this.webSocket){
            this.webSocket.close();
        }
    };

    chatClient.prototype.parseMessage = function parseMessage(rawMessage) {
        var parsedMessage = {
            message: null,
            tags: null,
            command: null,
            original: rawMessage,
            channel: null,
            username: null
        };
    
        if(rawMessage[0] === '@'){
            var tagIndex = rawMessage.indexOf(' '),
            userIndex = rawMessage.indexOf(' ', tagIndex + 1),
            commandIndex = rawMessage.indexOf(' ', userIndex + 1),
            channelIndex = rawMessage.indexOf(' ', commandIndex + 1),
            messageIndex = rawMessage.indexOf(':', channelIndex + 1);
    
            parsedMessage.tags = rawMessage.slice(0, tagIndex);
            parsedMessage.username = rawMessage.slice(tagIndex + 2, rawMessage.indexOf('!'));
            parsedMessage.command = rawMessage.slice(userIndex + 1, commandIndex);
            parsedMessage.channel = rawMessage.slice(commandIndex + 1, channelIndex);
            parsedMessage.message = rawMessage.slice(messageIndex + 1);
        } else if(rawMessage.startsWith("PING")) {
            parsedMessage.command = "PING";
            parsedMessage.message = rawMessage.split(":")[1];
        }
    
        return parsedMessage;
    }

    function generateTag(parsedMessage) {
      const container = document.createElement('div')
      const header = document.createElement('header')
      const text = document.createElement('p')

      header.textContent = parsedMessage.username
      text.textContent = parsedMessage.message
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

    chatClient = new chatClient({
        channel: '#stacking',
        username: 'Throwawaybot444',
        password: '',
    });