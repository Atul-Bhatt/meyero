import './People.css'
import Avatar from '@mui/material/Avatar'
import { API_ROUTES } from '../../utils/constants'
import axios from 'axios';

const People = ({ user, photoUrl, setCurrentUser, setCanvas}) => {
  const openChat = async (event) => {
		const token = localStorage.getItem("token");
		if (token) {
			axios.defaults.headers.common["Authorization"] = `Bearer ${token}`
		}
    event.preventDefault();

    try {
        const response = await axios.post(API_ROUTES.INITIATE_CHAT, {
          "id": "6add00c0-fbce-4846-95a1-f403f3f55fc3",
          "from_user": "6add00c0-fbce-4846-95a1-f403f3f55fc3",
          "to_user": user.id,
          "created_at": "2025-04-09T12:05:23+00:00",
          "updated_at": "2025-04-09T12:05:23+00:00",
    });
    console.log(response)
    let canvas = {
      received: response.data.receive_canvas,
      sent: response.data.send_canvas
    }
    setCanvas(canvas)
    } catch(error) {
        console.log(error);
    }

    setCurrentUser(user)
  };

  return (
    <div className='people__main' onClick={openChat}>
      <Avatar src={photoUrl} />
      <h3>{user.name}</h3>
    </div>
  )
}

export default People
