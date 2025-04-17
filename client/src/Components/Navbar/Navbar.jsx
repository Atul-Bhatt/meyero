import './Navbar.css';
import Avatar from '@mui/material/Avatar'

const Navbar = ({photoUrl, userName}) => {
    const handleLogOut = () => {
        localStorage.removeItem("token")
        localStorage.removeItem("userId")
        localStorage.removeItem("userName")
    }
	return (
    <nav className="navbar">
      <a href="/" className="navbar-logo">Meyero</a>

      <input type="text" className="navbar-search" placeholder="Search users..." />

      <div className="navbar-user">
        <Avatar src={photoUrl} />
        <div className="navbar-name">{localStorage.getItem("userName")}</div>
        <div className="navbar-dropdown">
          <a href="/profile">Profile</a>
          <a href="/login" onClick={handleLogOut}>Logout</a>
        </div>
      </div>
    </nav>
	);
};

export default Navbar;