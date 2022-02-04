import { gql, useApolloClient, useMutation, useQuery } from "@apollo/client";
import { Dispatch, SetStateAction, useState } from "react";
import { useParams } from "react-router-dom";
import { MY_ID } from "../constant";
import { GET_LEAGUE, UPDATE_LEAGUE } from "../graphql/League";
import { Card, CardBody, CardFooter, CardHeader, CardSubHeader } from "./Card";
import Loader from "./Loader";


const META_LEAGUE_STATE = gql`
query MetaLeagueState {
  metaLeagueState {
    values
    labels
  }
}
`;

const UPDATE_STATE = gql`
mutation UpdateState(
  $id: ID!
  $state: LeagueState
) {
  updateLeague(
    input:{
      id: $id
      state: $state
  }) {
    id
  }
}  
`;

type LeagueManageStateSelectProps = {
  current: String,
  selected: Dispatch<SetStateAction<string>>,
}

function LeagueManageStateSelect(props: LeagueManageStateSelectProps) {
  const { data } = useQuery(META_LEAGUE_STATE);

  if (data) {
    var states = []
    let index = 0;

    const values = data.metaLeagueState.values;
    const labels = data.metaLeagueState.labels;

    for (let i = 0; i < values.length; i++) {
      if (values[i] === props.current) {
        index = i;
      }

      states.push(
        <option key={values[i]} value={values[i]}>{labels[i]}</option>
      )
    }

    return (
      <div>
        <label htmlFor="state" className="px-4">League State</label>
        <select name="state" defaultValue={index} onChange={(e) => props.selected(e.target.value)}>
          {states}
        </select>
      </div>
    )
  } else {
    return null
  }
}

type LeagueManageProps = {
  league: any,
}

function LeagueManage(props: LeagueManageProps) {
  const [stateSelected, setStateSelected] = useState(props.league.state);
  
  const client = useApolloClient();
  const [error, setError] = useState("");

  const [update] = useMutation(UPDATE_LEAGUE, {
    variables: {
      id: props.league.id,
      state: stateSelected
    },
    onCompleted: ({ league }) => {
    	client.resetStore();
    },
    onError: ({message}) => {
      setError(message);
    }
  });


  function handleApply() {
    update({variables: {
        id: props.league.id,
	state: stateSelected
    }});
  }


  return (
    <div>
      <LeagueManageStateSelect current={stateSelected} selected={setStateSelected}/>
      <button
        className="btn p-3 my-2 bg-gray-700 text-paper rounded-sm border-b-4 border-paper w-full font-bold hover:bg-red-800"
        type="button"
        onClick={handleApply}
      >
        Apply
      </button>

    </div>
  )
}

type LeagueInfoProps = {
  league: any,
}

export default function LeagueInfo(props: LeagueInfoProps) {
  let my_id = localStorage.getItem(MY_ID);

  const [showManage, setShowManage] = useState(false);


  let players = props.league.teams.map((team: any) => {
    return (
      <li key={team.owner.username}>{team.owner.username} : {team.name} </li>
    )
  });

  const content = (
    <div>
      Owner: {props.league.owner.username}<br/><br/>

      Players: <ul>{players}</ul>

      <br/><br/>

      Public: {props.league.public.toString()}<br/>
      Max Players: {props.league.maxPlayers}<br/>
      State: {props.league.state}<br/>
      Manual: {props.league.manualState.toString()}<br/>
    </div>
  );

  return (
    <Card>
      <CardHeader title={"League: " + props.league.name} />
      <CardSubHeader content={props.league.description} />
      <CardBody>
        {content}
      </CardBody>
      { props.league.owner.id === my_id
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
          ? <LeagueManage league={props.league} />
          : null
          }
          </CardFooter>
        : null
      }
    </Card>
  )
}
