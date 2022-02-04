import {useQuery} from "@apollo/client";
import { useParams } from "react-router-dom";
import Drafting from "../components/Drafting";

import LeagueInfo from "../components/LeagueInfo";
import Loader from "../components/Loader";
import { TeamInfo } from "../components/TeamInfo";
import {MY_ID} from "../constant";
import {GET_LEAGUE} from "../graphql/League";

export default function League() {
  let { id } = useParams();
  //let my_id = localStorage.getItem(MY_ID);

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

  return (
    <div className="flex flex-col md:w-1/1 mx-auto p-8 space-y-8">
      <div className="flex flex-row space-x-4">
        <div className="basis-1/2"><LeagueInfo league={data.league} /></div>
        <div className="basis-1/2"><TeamInfo /></div>
      </div>
      {data.league.state === "DRAFTING"
        ? <Drafting league={data.league} />
        : null}
    </div>
  )
}
