import useWebSocket, { ReadyState } from "react-use-websocket"
import {useEffect} from 'react';
import JsonViewer from 'react-json-viewer';

export const WebSocket = () => {
	const WS_URL = "ws://localhost:8080"
	const { sendJsonMessage, lastJsonMessage, readyState } = useWebSocket(
		WS_URL,
		{
			share: false,
			shouldReconnect: () => true,
		},
	)

	// Run when the connection state (readyState) changes
	useEffect(() => {
		console.log("Connection state changed")
		if (readyState === ReadyState.OPEN) {
			sendJsonMessage({
				event: "subscribe",
				data: {
					channel: "general-chatroom",
				},
			})
		}
	}, [readyState])

	useEffect(() => {
		console.log(`Got a new message: ${JSON.stringify(lastJsonMessage)}`)
	}, [lastJsonMessage])

	const handleSendMessage = (event) => {
		if (event.key === 'Enter') {
			console.log('Sending message to server')
			sendJsonMessage({
				event: "subscribe",
				data: {
					channel: "general-chatroom",
					message: event.target.value 
				}
			})
		}
	}

	return (
        <div>
			<input
				type="text"
				onKeyDownCapture={handleSendMessage}>
			</input>
            <p>Message from server: {lastJsonMessage?.data.message}</p>
        </div>
    ) 
}

export default WebSocket;