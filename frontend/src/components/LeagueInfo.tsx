import { gql, useQuery } from "@apollo/client";
import { ObjectCanon } from "@apollo/client/cache/inmemory/object-canon";
import { useState } from "react";
import { useParams } from "react-router-dom";
import { MY_ID } from "../constant";
import { GET_LEAGUE } from "../graphql/League";
import { Card, CardBody, CardFooter, CardHeader, CardSubHeader } from "./Card";
import Loader from "./Loader";

const META_LEAGUE_STATE = gql`
query MetaLeagueState {
  metaLeagueState
}
`;

type LeagueManageStateSelectProps = {
  selected: String,
}

function LeagueManageStateSelect(props: LeagueManageStateSelectProps) {
  const { data } = useQuery(META_LEAGUE_STATE);

  if (data) {
    var states = []

    let selected = "";

    let k: keyof typeof data.metaLeagueState;
    for (k in data.metaLeagueState) {
      if (k === props.selected) {
        selected = k;
      }

      states.push(
        <option key={k} value={k}>{data.metaLeagueState[k]}</option>
      );
    }

    return (
      <div>
        <label htmlFor="state" className="px-4">League State</label>
        <select name="state" defaultValue={selected}>
          {states}
        </select>
      </div>
    )
  } else {
    return null
  }
}

type LeagueManageProps = {
  data: any,
}

function LeagueManage(props: LeagueManageProps) {
  return (
    <div>
      <LeagueManageStateSelect selected={props.data.league.state}/>
    </div>
  )
}

export default function LeagueInfo() {
  let { id } = useParams();
  let my_id = localStorage.getItem(MY_ID);

  const [showManage, setShowManage] = useState(false);

  let { loading, error, data } = useQuery(GET_LEAGUE, {
    variables: {id: id}
  });

  if (loading) {
    return (
      <div className="flex flex-col md:w-1/1 mx-auto p-8 space-y-8">
        <Loader />
      </div>
    )
  }

  if (error) {
    return (
      <div>{error}</div>
    )
  }

  let players = data.league.teams.map((team: any) => {
    return (
      <li key={team.owner.username}>{team.owner.username} : {team.name} </li>
    )
  });

  const content = (
    <div>
      Owner: {data.league.owner.username}<br/><br/>

      Players: <ul>{players}</ul>

      <br/><br/>

      Public: {data.league.public.toString()}<br/>
      Max Players: {data.league.maxPlayers}<br/>
      State: {data.league.state}<br/>
      Manual: {data.league.manualState.toString()}<br/>
    </div>
  );

  return (
    <Card>
      <CardHeader title={"League: " + data.league.name} />
      <CardSubHeader content={data.league.description} />
      <CardBody>
        {content}
      </CardBody>
      { data.league.owner.id === my_id
        ?
          <CardFooter>
          <button
            className="btn p-3 my-2 bg-gray-700 text-paper rounded-sm border-b-4 border-paper w-full font-bold hover:bg-red-800"
            type="button"
            onClick={() => setShowManage(!showManage)}
          >
            Manage
          </button>
          { showManage
          ? <LeagueManage data={data} />
          : null
          }
          </CardFooter>
        : null
      }
    </Card>
  )
}