import useWebSocket, { ReadyState } from "react-use-websocket"
import {useEffect} from 'react';
import Chat from "../Components/Chat/Chat";
const WEBSOCKET_URL = import.meta.env.VITE_WEBSOCKET_URL 

export const WebSocket = ({otherUser, canvas}) => {
	const token = localStorage.getItem('token')
	const socket_url = `ws://${WEBSOCKET_URL}/ws/user/${otherUser.id}?token=${token}`
	const { sendJsonMessage, lastJsonMessage, readyState } = useWebSocket(
		socket_url,
				{
			share: false,
			shouldReconnect: () => false,
		},
	)

	// Run when the connection state (readyState) changes
	// useEffect(() => {
	// 	console.log("Connection state changed")
	// 	if (readyState === ReadyState.OPEN) {
	// 		sendJsonMessage({
	// 			event: "subscribe",
	// 			data: {
	// 				channel: "general-chatroom",
	// 			},
	// 		})
	// 	}
	// }, [readyState])
	useEffect(() => {
		console.log(`Got a new message: ${JSON.stringify(lastJsonMessage?.message)}`)
	}, [lastJsonMessage])

	const handleSendMessage = (event) => {
		console.log('Sending message to server')
		sendJsonMessage({
			message: event.target.value
		})
	}

	return (
		<Chat lastJsonMessage={lastJsonMessage} 
			handleSendMessage={handleSendMessage}
			otherUser={otherUser}
			canvas={canvas}/>
    ) 
}

export default WebSocket;
