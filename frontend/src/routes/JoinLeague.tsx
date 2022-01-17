import { gql, useApolloClient, useMutation, useQuery } from "@apollo/client"
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useSetRecoilState } from "recoil";
import { selectedLeagueState } from "../components/App";
import { Card, CardBody, CardFooter, CardHeader } from "../components/Card";
import Loader from "../components/Loader";
import { MY_ID, SELECTED_LEAGUE } from "../constant";
import { JOIN_LEAGUE } from "../graphql/League";


const PUBLIC_LEAGUES = gql`
query Leagues (
  $excludeId: ID!
  $search: String
) {
  leagues(filter: {
    _and: [
      {public: true},
      {owner: {
      	_ne: $excludeId
      }},
      {managers: {
        _ne: $excludeId
      }},
      {name: {
        _regex: $search
        _options: "i"
      }}
    ]
  }) {
    id
    name
    description
    maxPlayers,
    state,
    manualState,
    managersCount,
    owner {
      username
    }
    managers {
      username
    }
  }
}
`;

type LeagueListProps = {
  search: string,
}

function LeagueList(props: LeagueListProps) {
  const my_id = localStorage.getItem(MY_ID);

  const { loading, error, data } = useQuery(PUBLIC_LEAGUES, {
    fetchPolicy: "network-only",
    variables: {
      excludeId: my_id,
      search: props.search,
    }
  });

  const setSelectedLeague = useSetRecoilState(selectedLeagueState);

  const navigate = useNavigate();
  const client = useApolloClient();

  const [ joinLeague ] = useMutation(JOIN_LEAGUE, {
    fetchPolicy: "network-only",
    onCompleted: ({ joinLeague }) => {
      localStorage.setItem(SELECTED_LEAGUE, joinLeague.id);
      setSelectedLeague(joinLeague.id);
      client.resetStore();
      navigate(`/league/${joinLeague.id}`)
    },
    onError: ({message}) => {
      console.error(message);
    }
  });

  if (loading) {
    return (
      <Loader />
    )
  }

  if (error) {
    return (
      <div className="flex flex-col md:w-1/3 mx-auto p-8 space-y-8">
        {error}
      </div>
    )
  }

  function handleJoin(id: string) {
    joinLeague({variables: {id: id}});
  };

  const leagues = data.leagues.map((league: any) => {
    return (
      <Card key={league.id}>
        <CardHeader title={league.name}/>
        <CardBody>
          Owner: {league.owner.username} <br/>
          Description: {league.description} <br/>
        </CardBody>
        <CardFooter>
          <button
            className="align-self-end btn py-1 px-8 bg-gray-700 text-paper rounded-sm border-b-4 border-paper font-bold hover:bg-red-800 float-right"
            onClick={() => handleJoin(league.id)}
          >
            Join
          </button>
        </CardFooter>
      </Card>
    )
  })

  let notFound = props.search ? "No leagues found" : "No public leagues to join";

  return (
    <div>
      {data.leagues.length === 0 ? <h1>{notFound}</h1> : null}
      {leagues}
    </div>
  )
}

export default function JoinLeague() {
  const [ search, setSearch ] = useState("");

  return (
    <div className="flex flex-col md:w-1/3 mx-auto p-8 space-y-8">
      <input
        value={search}
        onChange={(e: any) => setSearch(e.target.value)}
        placeholder="Search league name..."
        className="p-2 border-0 border-b-2 border-black outline-none bg-paper focus:ring focus:ring-red-700"
      />
      <LeagueList search={search} />
    </div>
  )
}