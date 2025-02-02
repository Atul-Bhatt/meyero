import useWebSocket, { ReadyState } from "react-use-websocket"
import {useEffect} from 'react';
import Chat from "../Components/Chat/Chat";

export const WebSocket = () => {
	const WS_URL = process.env.WEBSOCKET_URL
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
		<Chat lastJsonMessage={lastJsonMessage} 
			handleSendMessage={handleSendMessage}/>
    ) 
}

export default WebSocket;
