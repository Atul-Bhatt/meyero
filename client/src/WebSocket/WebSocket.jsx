import useWebSocket, { ReadyState } from "react-use-websocket"
import {useEffect} from 'react';
import Chat from "../Components/Chat/Chat";

export const WebSocket = ({currentUser, canvas}) => {
	const token = localStorage.getItem('token')
	const socket_url = `ws://localhost:8080/ws/user/${currentUser.id}?token=${token}`
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
		console.log(`Got a new message: ${JSON.stringify(lastJsonMessage)}`)
	}, [lastJsonMessage])

	const handleSendMessage = (event) => {
		if (event.key === 'Enter') {
			console.log('Sending message to server')
			sendJsonMessage({
				message: event.target.value
			})
		}
	}

	return (
		<Chat lastJsonMessage={lastJsonMessage} 
			handleSendMessage={handleSendMessage}
			currentUser={currentUser}
			canvas={canvas}/>
    ) 
}

export default WebSocket;
