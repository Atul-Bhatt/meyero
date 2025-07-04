const API_URL = 'http://localhost:8081'
export const API_ROUTES = {
  SIGN_UP: `${API_URL}/user/signup`,
  LOG_IN: `${API_URL}/user/login`,
  GET_USER_LIST: `${API_URL}/user/list`,
  INITIATE_CHAT: `${API_URL}/messaging/initiate`,
  SEARCH: `${API_URL}/user/search`,
}

export const APP_ROUTES = {
  SIGN_UP: '/signup',
  LOG_IN: '/login',
  HOME: '/',
  WEBSOCKET: '/websocket',
}
