const io = require('socket.io-client')

export default function () {
	cosnt socket = io.connect('http://localhost:8080')

	function register(name, cb) {
		socket.emit('register', name, cb)
	}

	function message(chatroomName, msg, cb) {
		socket.emit('message', { chatroomName, message: msg }, cb)
	}

	return {
		register,
		message
	}
}
