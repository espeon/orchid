let socket = new WebSocket("ws://localhost:3000/ws");

socket.addEventListener("open", function (event) {
  socket.send("Hello Server!");
});

socket.addEventListener("message", function (event) {
  console.log("Message from server:", event.data);
});

function handleError(error) {
  console.error("Socket Error: ", error);
  if (error.code === 1006) {
    console.log("Connection closed, trying to reconnect");
    handleReconnect();
  } else if (error.code === 1001) {
    console.log("Connection error");
  } else {
    console.log("Unknown error");
  }
}

function handleReconnect() {
  try {
    console.log("Attempting to reconnect");
    socket = new WebSocket("ws://localhost:3000/ws");
    socket.addEventListener("error", function (event) {
      console.error("Error reconnecting: ", event);
      console.log("Trying again in 5 seconds");

      setTimeout(() => {
        handleReconnect();
      }, 5000);
    });
    socket.addEventListener("open", function (event) {
      socket.send("Hello Server!");
    });
    socket.addEventListener("message", function (event) {
      console.log("Message from server:", event.data);
    });
  } catch (e) {
    console.log("Error reconnecting: ", e);
    console.log("Trying again in 5 seconds");

    setTimeout(() => {
      handleReconnect();
    }, 5000);
  }
}

socket.addEventListener("close", handleError);
socket.addEventListener("error", handleError);
socket.addEventListener("reconnect", handleReconnect);

function sendText(socket, text) {
  try {
    socket.send(text);
  } catch (e) {
    console.log("Error sending message: ", e);
  }
}

setTimeout(() => {
  socket.send("echo hello world minecraft parody");
}, 1000);

// setTimeout(() => {
//   socket.send("About done here...");
//   console.log("Sending close over websocket");
//   socket.close(3000, "Crash and Burn!");
// }, 3000);
