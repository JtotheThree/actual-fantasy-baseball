import toTitleCase from "../utils";
import { Card, CardBody, CardHeader, CardHeaderLink, CardSubHeader } from "./Card";
import { HoverInfo } from "./HoverInfo";

type RosterProps = {
  roster: any,
}

export default function Roster(props: RosterProps) {
  let roster = props.roster

  function PlayerInfoHover(player: any) {
    let data = (player)
      ? (<HoverInfo text={player.name}>
          <div className="w-96">
          <Card>
            <CardHeader title={player.name} />
            <CardSubHeader content={player.class + " " + player.race + " " + player.gender + " " + (player.handedness === "LEFT" ? "Lefty" : "Righty" )} />
            <CardBody>
              <table className="border-separate table-auto">
                <thead>
                  <tr>
                    <th>STR</th>
                    <th>DEX</th>
                    <th>CON</th>
                    <th>WIS</th>
                    <th>INT</th>
                    <th>CHR</th>
                    <th>HP</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td>{player.strength}</td>
                    <td>{player.dexterity}</td>
                    <td>{player.constitution}</td>
                    <td>{player.wisdom}</td>
                    <td>{player.intelligence}</td>
                    <td>{player.charisma}</td>
                    <td>{player.maxHealth}</td>
                  </tr>
                </tbody>
              </table>
              <br></br>
              <span>{player.traits[0]} | {player.traits[1]}</span>
            </CardBody>
          </Card>
          </div>
        </HoverInfo>)
      : "None";

    return data;
  }

  let rosterTable = (
    <div>
      <table>
        <tr>
          <td>SP</td>
          <td>{PlayerInfoHover(roster.startingPitcher)}</td>
        </tr>
        <tr>
          <td>C</td>
          <td>{PlayerInfoHover(roster.catcher)}</td>
        </tr>
        <tr>
          <td>1B</td>
          <td>{PlayerInfoHover(roster.firstBase)}</td>
        </tr>
        <tr>
          <td>2B</td>
          <td>{PlayerInfoHover(roster.secondBase)}</td>
        </tr>
        <tr>
          <td>3B</td>
          <td>{PlayerInfoHover(roster.thirdBase)}</td>
        </tr>
        <tr>
          <td>SS</td>
          <td>{PlayerInfoHover(roster.shortstop)}</td>
        </tr>
        <tr>
          <td>LF</td>
          <td>{PlayerInfoHover(roster.leftField)}</td>
        </tr>
        <tr>
          <td>CF</td>
          <td>{PlayerInfoHover(roster.centerField)}</td>
        </tr>
        <tr>
          <td>RF</td>
          <td>{PlayerInfoHover(roster.rightField)}</td>
        </tr>
      </table>
      <table>
        <tr>
          <td>Relief Pitchers</td>
          <td>{roster.reliefPitchers.map(function(player: any) {return PlayerInfoHover(player)})}</td>
        </tr>
        <tr>
          <td>Catcher Reserves</td>
          <td>{roster.catcherReserves.map(function(player: any) {return PlayerInfoHover(player)})}</td>
        </tr>
        <tr>
          <td>Infield Reserves</td>
          <td>{roster.infieldReserves.map(function(player: any) {return PlayerInfoHover(player)})}</td>
        </tr>
        <tr>
          <td>Outfield Reserves</td>
          <td>{roster.outfieldReserves.map(function(player: any) {return PlayerInfoHover(player)})}</td>
        </tr>
      </table>
    </div>
  )

  return (
    <div>
      {rosterTable}
    </div>
  )
}