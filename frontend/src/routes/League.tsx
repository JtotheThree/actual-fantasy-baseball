import LeagueInfo from "../components/LeagueInfo";
import { TeamInfo } from "../components/TeamInfo";


export default function League() {
  return (
    <div className="flex flex-col md:w-1/1 mx-auto p-8 space-y-8">
      <div className="flex flex-row space-x-4">
        <div className="basis-1/2"><LeagueInfo /></div>
        <div className="basis-1/2"><TeamInfo /></div>
      </div>
    </div>
  )
}