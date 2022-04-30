import { gql, useApolloClient, useMutation, useQuery } from "@apollo/client"
import {Dispatch, SetStateAction, useState} from "react";
import Modal from 'react-modal';
import { Card, CardBody, CardHeader } from "./Card";
import Loader from "./Loader";

import toTitleCase from "../utils";
import {DebounceInput} from "react-debounce-input";
import {useParams} from "react-router-dom";
import {MY_ID} from "../constant";
import {MY_TEAM_FOR_LEAGUE} from "../graphql/Team";

const META_CLASS = gql`
query MetaClass {
  metaClass {
    values
    labels
  }
}
`;

const META_RACE = gql`
query MetaRace {
  metaRace {
    values
    labels
  }
}
`;


const GET_PLAYERS_FOR_DRAFTING = gql`
query GetPlayersForDrafting (
  $league: String!
  $class: String!
  $race: String!
  $strength: Int!
  $dexterity: Int!
  $constitution: Int!
  $intelligence: Int!
  $wisdom: Int!
  $charisma: Int!
) {
  players(filter: {
    league: $league
    team: null
    class: {_regex: $class},
    race: {_regex: $race},
    _and: [
      {strength: {_gt: $strength}}
      {dexterity: {_gt: $dexterity}}
      {constitution: {_gt: $constitution}}
      {intelligence: {_gt: $intelligence}}
      {wisdom: {_gt: $wisdom}}
      {charisma: {_gt: $charisma}}
      {maxHealth: {_gt: 0}}
    ]
  }) {
    id
    name
    cost
    gender
    race
    class
    maxHealth
    strength
    dexterity
    constitution
    intelligence
    wisdom
    charisma
    traits
  }
}
`;

const DRAFT_PLAYER = gql`
mutation DraftPlayer(
  $teamId: String!
  $playerId: String!
) {
  setTeam(
    id: $playerId
    teamId: $teamId
  ) {
    name
    team {
      name
    }
  }
}
`;

const MODIFY_GOLD = gql `
mutation ModifyGold(
  $teamId: ID!
  $cost: Int!
) {
  modifyGold(
    id: $teamId
    cost: $cost
  ) {
    name
  }
}
`;

type DraftingClassFilterProps = {
  selected: String,
  onChange: any,
}

function DraftingClassFilter(props: DraftingClassFilterProps) {
  const { data } = useQuery(META_CLASS);

  if (data) {
    var classes = []
    let index = 0;

    const values = data.metaClass.values;
    const labels = data.metaClass.labels;

    classes.push(
      <option key="ANY" value=".*">Any</option>
    );

    for (let i = 0; i < values.length; i++) {
      if (values[i] === props.selected) {
        index = i;
      }

      classes.push(
        <option key={values[i]} value={values[i]}>{labels[i]}</option>
      )
    }

    return (
      <div>
        <select name="classFilter" defaultValue={index} onChange={props.onChange}> 
          {classes}
        </select>
      </div>
    )
  } else {
    return null
  }
}

type DraftingRaceFilterProps = {
  selected: String,
  onChange: any,
}

function DraftingRaceFilter(props: DraftingRaceFilterProps) {
  const { data } = useQuery(META_RACE);

  if (data) {
    var races = []
    let index = 0;

    const values = data.metaRace.values;
    const labels = data.metaRace.labels;

    races.push(
      <option key="ANY" value=".*">Any</option>
    );

    for (let i = 0; i < values.length; i++) {
      if (values[i] === props.selected) {
        index = i;
      }

      races.push(
        <option key={values[i]} value={values[i]}>{labels[i]}</option>
      )
    }

    return (
      <div>
        <select name="classFilter" defaultValue={index} onChange={props.onChange}> 
          {races}
        </select>
      </div>
    )
  } else {
    return null
  }
}

type PlayerRowProps = {
  player: any,
  team: string,
}

function PlayerRow(props: PlayerRowProps) {
  const [modalIsOpen, setIsOpen] = useState(false);
  const [error, setError] = useState("");
  const client = useApolloClient();

  const [draft] = useMutation(DRAFT_PLAYER, {
    variables: {
      teamId: props.team,
      playerId: props.player.id,
    },
    onCompleted: ({ team }) => {
      console.log(team);
    },
    onError: ({message}) => {
      console.error(message);
      setError(message);
    }
  });

  const [gold] = useMutation(MODIFY_GOLD, {
    variables: {
      teamId: props.team,
      cost: -props.player.cost,
    }
  });

  let player = props.player;

  function openModal() {
    setIsOpen(true);
  }

  function closeModal() {
    setIsOpen(false);
  }

  function onConfirm() {
    draft();
    gold();
    client.resetStore();
    setIsOpen(false);
  }

  const styles = {
    content: {
      top: '50%',
      left: '50%',
      right: 'auto',
      bottom: 'auto',
      marginRight: '50%',
      transform: 'translate(-50%, -50%)',
      backgroundColor: '#f9f8f3',
    },
  };

  return (
    <>
    {error}
    <tr key={player.id}>
      <td className="py-4 text-2xl">{player.name}</td>
      <td>{toTitleCase(player.class)}</td>
      <td>{toTitleCase(player.race)}</td>
      <td className="font-bold text-lg">{player.strength}</td>
      <td className="font-bold text-lg">{player.dexterity}</td>
      <td className="font-bold text-lg">{player.constitution}</td>
      <td className="font-bold text-lg">{player.wisdom}</td>
      <td className="font-bold text-lg">{player.intelligence}</td>
      <td className="font-bold text-lg">{player.charisma}</td>
      <td className="w-24">{player.maxHealth}</td>
      <td className="w-64">{toTitleCase(player.traits[0])}, {toTitleCase(player.traits[1])}</td>
      <td className="font-bold text-lg">{player.cost}</td>
      <td><button className="px-3 bg-gray-700 text-paper font-bold" onClick={openModal}>+</button></td>
    </tr>
    <Modal
      isOpen={modalIsOpen}
      onRequestClose={closeModal}
      style={styles}
      contentLabel="Example Model"
    >
      <div className="bg-paper">
        <h2 className="font-bold text-xl">Draft {player.name}?</h2>
          <div className="flex">
            <h4>{toTitleCase(player.class)}</h4>
            <h4 className="mx-2">{toTitleCase(player.race)}</h4>
          </div>
          <br></br>
          <table className="min-w-full divide-y divide-gray-200">
            <thead>
              <tr>
                <th className="text-left">STR</th>
                <th className="text-left">DEX</th>
                <th className="text-left">CON</th>
                <th className="text-left">WIS</th>
                <th className="text-left">INT</th>
                <th className="text-left">CHR</th>
                <th className="text-left">HP</th>
                <th className="text-left">TRAITS</th>
                <th></th>
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
                <td>{player.traits}</td>
              </tr>
            </tbody>
          </table>
        <br></br>
        <span className="text-lg font-bold">For: {player.cost} Gold</span>
        <div className="flex">
          <button className="btn p-3 my-2 bg-gray-700 text-paper rounded-sm border-b-4 border-paper w-full font-bold hover:bg-red-800" onClick={closeModal}>Cancel</button>
          <button className="btn mx-4 p-3 my-2 bg-gray-700 text-paper rounded-sm border-b-4 border-paper w-full font-bold hover:bg-red-800" onClick={onConfirm}>Confirm</button>
        </div>
      </div>
    </Modal>
    </>
  )
}

type DraftingPlayerListProps = {
  league: string,
  team: string,
  classFilter: string,
  raceFilter: string,
  strengthFilter: number,
  dexterityFilter: number,
  constitutionFilter: number,
  intelligenceFilter: number,
  wisdomFilter: number,
  charismaFilter: number,
  costSort: number,
}

function DraftingPlayerList(props: DraftingPlayerListProps) {
   let { loading, error, data } = useQuery(GET_PLAYERS_FOR_DRAFTING, {
    variables: {
      league: props.league,
      class: props.classFilter,
      race: props.raceFilter,
      strength: props.strengthFilter,
      dexterity: props.dexterityFilter,
      constitution: props.constitutionFilter,
      intelligence: props.intelligenceFilter,
      wisdom: props.wisdomFilter,
      charisma: props.charismaFilter,
    }
  });

  if (loading) {
    return (
      <tbody className="flex flex-col md:w-1/1 mx-auto p-8 space-y-8">
        <tr>
          <td>Loading...</td>
        </tr>
      </tbody>
    )
  }

  if (error) {
    return (
      <tbody>
        <tr>
          {error}
        </tr>
      </tbody>
    )
  }

  const players = data.players.map((player: any) => {
    return (
      <PlayerRow player={player} team={props.team} key={player.id} />
    )
  })


  return (
    <tbody className="divide-y divide-gray-200">
      {players}
    </tbody>
  )
}

type DraftingAttribProps = {
  state: React.Dispatch<React.SetStateAction<number>>
  default: number
}

function DraftingAttrib(props: DraftingAttribProps) {
  return (
      <DebounceInput
        debounceTimeout={300}
        className="mx-2 w-1/3 text-lg border-0 border-b-2 border-black outline-none bg-paper focus:ring focus:ring-red-700"
        type="number"
        min={0}
        max={30}
        value={props.default}
        onChange={(e) => props.state(parseInt(e.target.value))}
      />
  )
}

type DraftingProps = {
  league: any
}

export default function Drafting(props: DraftingProps) {
  const [classFilter, setClassFilter] = useState(".*");
  const [raceFilter, setRaceFilter] = useState(".*");
  const [strengthFilter, setStrengthFilter] = useState(0);
  const [dexterityFilter, setDexterityFilter] = useState(0);
  const [constitutionFilter, setConstitutionFilter] = useState(0);
  const [intelligenceFilter, setIntelligenceFilter] = useState(0);
  const [wisdomFilter, setWisdomFilter] = useState(0);
  const [charismaFilter, setCharismaFilter] = useState(0);

  const my_id = localStorage.getItem(MY_ID);

  const { loading, error, data } = useQuery(MY_TEAM_FOR_LEAGUE, {
    variables: {
      ownerId: my_id,
      leagueId: props.league.id,
    }
  });

  if (loading) {
    return (
      <Loader />
    )
  }

  if (error) {
    console.error(error);
  }



  function classFilterChange(e: any) {
    setClassFilter(e.target.value);
  }

  function raceFilterChange(e: any) {
    setRaceFilter(e.target.value);
  }

  console.log(data.teams[0].id)

  return (
    <Card>
      <CardHeader title={"Drafting"} />
      <CardBody>
        <table className="min-w-full divide-y divide-gray-200">
          <thead>
            <tr>
              <th className="text-left">NAME</th>
              <th className="text-left">CLASS</th>
              <th className="text-left">RACE</th>
              <th className="text-left">STR</th>
              <th className="text-left">DEX</th>
              <th className="text-left">CON</th>
              <th className="text-left">WIS</th>
              <th className="text-left">INT</th>
              <th className="text-left">CHR</th>
              <th className="text-left">HP</th>
              <th className="text-left">TRAITS</th>
              <th className="text-left">COST</th>
              <th></th>
            </tr>
            <tr>
              <th className="text-left"></th>
              <th className="text-left w-24"><DraftingClassFilter selected={classFilter} onChange={classFilterChange} /></th>
              <th className="text-left w-24"><DraftingRaceFilter selected={raceFilter} onChange={raceFilterChange} /></th>
              <th className="text-left">&lt; <DraftingAttrib default={strengthFilter} state={setStrengthFilter} /></th>
              <th className="text-left">&lt; <DraftingAttrib default={dexterityFilter} state={setDexterityFilter} /></th>
              <th className="text-left">&lt; <DraftingAttrib default={constitutionFilter} state={setConstitutionFilter} /></th>
              <th className="text-left">&lt; <DraftingAttrib default={intelligenceFilter} state={setIntelligenceFilter} /></th>
              <th className="text-left">&lt; <DraftingAttrib default={wisdomFilter} state={setWisdomFilter} /></th>
              <th className="text-left">&lt; <DraftingAttrib default={charismaFilter} state={setCharismaFilter} /></th>
              <th className="text-left"></th>
              <th className="text-left"></th>
              <th className="text-left"><input type="number" min="0" value="0" readOnly /></th>
              <th></th>
            </tr>
          </thead>
            <DraftingPlayerList 
              league={props.league.id} 
              team={data.teams[0].id}
              classFilter={classFilter} 
              raceFilter={raceFilter}
              strengthFilter={strengthFilter}
              dexterityFilter={dexterityFilter}
              constitutionFilter={constitutionFilter}
              intelligenceFilter={intelligenceFilter}
              wisdomFilter={wisdomFilter}
              charismaFilter={charismaFilter}
              costSort={0}
            />
        </table>
      </CardBody>
    </Card>
  )
}
