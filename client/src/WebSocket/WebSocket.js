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
		console.log(`Got a new message: ${lastJsonMessage}`)
	}, [lastJsonMessage])

	return (
        <div>
            <p>Hello</p>
            <JsonViewer data={lastJsonMessage} />
        </div>
    ) 
}

export default WebSocket;