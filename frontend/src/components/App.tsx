import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Modal from 'react-modal';
import { atom } from 'recoil';

import {AUTH_TOKEN, SELECTED_LEAGUE} from '../constant'

import Header from './Header';
import Home from '../routes/Home';
import Login from '../routes/Login';
import Signup from '../routes/Signup';
import League from '../routes/League';
import CreateLeague from '../routes/CreateLeague';
import JoinLeague from '../routes/JoinLeague';
import Team from '../routes/Team';


export const tokenState = atom({
  key: 'tokenState',
  default: localStorage.getItem(AUTH_TOKEN),
});

export const selectedLeagueState = atom({
  key: 'selectedLeageState',
  default: localStorage.getItem(SELECTED_LEAGUE),
});

Modal.setAppElement("#root");

function App() {
  return (
    <div>
      <BrowserRouter>
        <Header />
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/createLeague" element={<CreateLeague />} />
          <Route path="/joinLeague" element={<JoinLeague />} />
          <Route path="/league/:id" element={<League />} />
          <Route path="/login" element={<Login />} />
          <Route path="/signup" element={<Signup />} />
          <Route path="/team/:id" element={<Team />} />
        </Routes>
      </BrowserRouter>
    </div>
  );
}


export default App;
