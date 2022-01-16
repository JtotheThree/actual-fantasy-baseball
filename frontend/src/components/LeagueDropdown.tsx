import { useQuery } from '@apollo/client';
import { Link, useNavigate } from 'react-router-dom';
import { useRecoilState, useRecoilValue } from 'recoil';

import { Dropdown, DropdownItem } from '../components/Dropdown';
import { SELECTED_LEAGUE } from '../constant';
import { ME } from '../graphql/auth';
import { selectedLeagueState } from './App';

function Selected(id: string, name: string) {
  return (
    <Link to={`/league/${id}`}>
      {name}
    </Link>
  )
}

type LeagueDropdownItemProps = {
  id: string
  to: string
  name: string
  className?: string
}

function LeagueDropdownItem(props: LeagueDropdownItemProps) {
  return (
    <DropdownItem id={props.id} className={props.className + " block p-4 text-lg text-bold hover:text-red-800"}>
      <Link to={props.to}>
        {props.name}
      </Link>
    </DropdownItem>
  )
}

export default function LeagueDropdown() {
  const { data } = useQuery(ME);
  const [ stateSelectedLeague, setSelectedLeague ] = useRecoilState(selectedLeagueState);
  const navigate = useNavigate();

  if (!data) {
    return (
      <div></div>
    )
  }

  if (data.me.joinedLeagues.length === 0) {
    return (
      <span className="text-lg text-center font-bold">
        <Link to="/createLeague" className="underline">
          Create
        </Link>
        {" or "}
        <Link to="/joinLeague" className="underline">
          Join
        </Link>
        {" a league"}
      </span>
    )
  }

  function handleSelected(id: string) {
    localStorage.setItem(SELECTED_LEAGUE, id);
    setSelectedLeague(id);
    navigate(`/league/${id}`);
  }

  let selectedLeague = null;

  const leagues = data.me.joinedLeagues.map((league: any) => {
    if (stateSelectedLeague) {
      if (stateSelectedLeague === league.id) {
        selectedLeague = Selected(league.id, league.name);
      }
    }

    return (
      // TODO: Convert to LeagueDropdownItem
      <DropdownItem key={league.id} id={league.id} onClick={handleSelected} className="block p-4 text-lg font-bold hover:text-red-800">
        {league.name}
      </DropdownItem>
    )
  });

  if (!selectedLeague) {
    selectedLeague = Selected(data.me.joinedLeagues[0].id, data.me.joinedLeagues[0].name);
  }

  return (
    <Dropdown selected={selectedLeague}>
      {leagues}
      <LeagueDropdownItem name="Create a League" className="border-t-2" id="create" to="/createLeague" />
      <LeagueDropdownItem name="Join a League" id="join" to="/joinLeague" />
    </Dropdown>
  )
}