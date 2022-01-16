import { gql, useApolloClient, useQuery } from "@apollo/client";
import { MouseEvent } from "react";
import { Link, useNavigate } from "react-router-dom";
import { useRecoilValue, useSetRecoilState } from "recoil";

import {tokenState} from '../components/App';
import LeagueDropdown from '../components/LeagueDropdown';
import { AUTH_TOKEN } from "../constant";
import { ME } from "../graphql/auth";

type HeaderLinkProps = {
  name: string,
  to: string
}

function HeaderLink(props: HeaderLinkProps) {
  return (
    <li>
      <Link to={props.to} className="md:p-4 py-2 block hover:text-red-800">
        {props.name}
      </Link>
    </li>
  )
}

function HeaderRightAuth() {
  const {data} = useQuery(ME);
  const navigate = useNavigate();
  const client = useApolloClient();
  const setToken = useSetRecoilState(tokenState);

  const handleLogout = (event: MouseEvent) => {
    localStorage.removeItem(AUTH_TOKEN);
    setToken(null);
    client.clearStore();
    navigate("/");
  };

  return (
    <>
    {data ? <HeaderLink to="/" name={data.me.username} /> : null}
    <li>
      <button onClick={handleLogout} className="md:p-4 py-2 block font-bold hover:text-red-800">
        Logout
      </button>
    </li>
    </>
  )
}

function HeaderRightGuest() {
  return (
    <>
    <HeaderLink to="/signup" name="Signup" />
    <HeaderLink to="/login" name="Login" />
    </>
  )
}

export default function Header() {
  const token = useRecoilValue(tokenState);

  return (
    <header className="border-solid border-b-2 border-black shadow-md">
      <nav className="flex flex-wrap items-center justify-between w-full py-4 md:py-0 px-4 text-2xl bg-paper">
        <ul className="flex flex-wrap">
          <Link to="/" className="font-title">Actual Fantasy Baseball</Link>
          <div className="md:px-6 px-6">
            {token ? <LeagueDropdown /> : null}
          </div>
        </ul>
        <ul className="pt-4 text-base md:flex md:justify-between md:pt-0 font-bold">
        {token ? <HeaderRightAuth /> : <HeaderRightGuest />}
        </ul>
      </nav>
    </header>
  )
}
