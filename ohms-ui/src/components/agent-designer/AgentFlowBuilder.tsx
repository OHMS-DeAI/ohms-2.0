import { useCallback, useEffect, useMemo, useRef, useState } from 'react'
import type { DragEvent } from 'react'
import ReactFlow, {
  Background,
  Controls,
  MarkerType,
  MiniMap,
  Position,
  addEdge,
  Handle,
  useEdgesState,
  useNodesState,
} from 'reactflow'
import type { Connection, Edge, Node, NodeProps, ReactFlowInstance } from 'reactflow'
import Input from '../Input'
import Textarea from '../Textarea'
import Button from '../Button'
import 'reactflow/dist/style.css'

type FlowNodeKind = 'trigger' | 'agent' | 'tool' | 'condition' | 'output'

export interface AgentFlowNodeData {
  title: string
  description: string
  flowType: FlowNodeKind
}

export interface AgentFlowState {
  nodes: Node<AgentFlowNodeData>[]
  edges: Edge[]
}

interface AgentFlowBuilderProps {
  onFlowChange?: (state: AgentFlowState) => void
  initialState?: Partial<AgentFlowState>
}

const nodeTemplateDescriptors: Array<{
  flowType: FlowNodeKind
  title: string
  description: string
}> = [
  {
    flowType: 'trigger',
    title: 'Trigger',
    description: 'Start the workflow when an event happens.',
  },
  {
    flowType: 'agent',
    title: 'Agent Action',
    description: 'Delegate a reasoning task to an agent persona.',
  },
  {
    flowType: 'tool',
    title: 'Tool Call',
    description: 'Call an external service or dataset.',
  },
  {
    flowType: 'condition',
    title: 'Decision Gate',
    description: 'Route based on outputs or scores.',
  },
  {
    flowType: 'output',
    title: 'Finalize',
    description: 'Wrap up the workflow and ship results.',
  },
]

const accentByFlowType: Record<FlowNodeKind, string> = {
  trigger: 'border-amber-400/70 bg-amber-500/10',
  agent: 'border-cyan-400/70 bg-cyan-500/10',
  tool: 'border-emerald-400/70 bg-emerald-500/10',
  condition: 'border-purple-400/70 bg-purple-500/10',
  output: 'border-rose-400/70 bg-rose-500/10',
}

const labelByFlowType: Record<FlowNodeKind, string> = {
  trigger: 'Trigger',
  agent: 'Agent',
  tool: 'Tool',
  condition: 'Decision',
  output: 'Output',
}

const buildInitialNodes = (): Node<AgentFlowNodeData>[] => [
  {
    id: 'trigger-1',
    position: { x: 0, y: 0 },
    data: {
      flowType: 'trigger',
      title: 'Kick-off',
      description: 'Start when this workflow is launched from the dashboard.',
    },
    type: 'agentCard',
  },
  {
    id: 'agent-1',
    position: { x: 0, y: 160 },
    data: {
      flowType: 'agent',
      title: 'Plan mission',
      description: 'Draft the steps required to satisfy the user goal.',
    },
    type: 'agentCard',
  },
  {
    id: 'output-1',
    position: { x: 0, y: 320 },
    data: {
      flowType: 'output',
      title: 'Report outcome',
      description: 'Summarize results and notify the requesting user.',
    },
    type: 'agentCard',
  },
]

const buildInitialEdges = (): Edge[] => [
  {
    id: 'e-trigger-1->agent-1',
    source: 'trigger-1',
    target: 'agent-1',
    animated: true,
    markerEnd: {
      type: MarkerType.ArrowClosed,
      color: '#facc15',
    },
    style: { stroke: '#facc15', strokeWidth: 2 },
  },
  {
    id: 'e-agent-1->output-1',
    source: 'agent-1',
    target: 'output-1',
    animated: true,
    markerEnd: {
      type: MarkerType.ArrowClosed,
      color: '#f97316',
    },
    style: { stroke: '#f97316', strokeWidth: 2 },
  },
]

const AgentFlowNode = ({ data, selected }: NodeProps<AgentFlowNodeData>) => {
  const colors = accentByFlowType[data.flowType]
  const hasTarget = data.flowType !== 'trigger'
  const hasSource = data.flowType !== 'output'

  return (
    <div
      className={`rounded-lg border px-4 py-3 shadow-md text-left transition ring-2 ring-transparent ${
        colors
      } ${selected ? 'ring-accentGold' : ''}`}
    >
      {hasTarget && (
        <Handle
          type="target"
          position={Position.Top}
          style={{ background: 'rgba(250, 204, 21, 0.65)', border: 'none', width: 12, height: 12 }}
        />
      )}
      <p className="text-xs font-semibold uppercase tracking-wide text-textOnDark/60">
        {labelByFlowType[data.flowType]}
      </p>
      <p className="text-base font-semibold text-textOnDark mt-1">{data.title}</p>
      {data.description && <p className="text-sm text-textOnDark/70 mt-1 leading-snug">{data.description}</p>}
      {hasSource && (
        <Handle
          type="source"
          position={Position.Bottom}
          style={{ background: 'rgba(250, 204, 21, 0.65)', border: 'none', width: 12, height: 12 }}
        />
      )}
    </div>
  )
}

const nodeTypes = { agentCard: AgentFlowNode }

export const generateInstructionFromFlow = (state: AgentFlowState): string => {
  if (!state.nodes.length) return ''

  const nodeMap = new Map(state.nodes.map(node => [node.id, node]))
  const outgoing = new Map<string, Edge[]>(state.nodes.map(node => [node.id, []]))
  const incomingCount = new Map<string, number>(state.nodes.map(node => [node.id, 0]))

  state.edges.forEach(edge => {
    outgoing.get(edge.source)?.push(edge)
    incomingCount.set(edge.target, (incomingCount.get(edge.target) ?? 0) + 1)
  })

  const orderedNodes: Node<AgentFlowNodeData>[] = []
  const visited = new Set<string>()

  const walk = (node: Node<AgentFlowNodeData>) => {
    if (visited.has(node.id)) return
    visited.add(node.id)
    orderedNodes.push(node)
    const edges = outgoing.get(node.id) ?? []
    edges
      .map(edge => nodeMap.get(edge.target))
      .filter((n): n is Node<AgentFlowNodeData> => Boolean(n))
      .sort((a, b) => a.position.y - b.position.y)
      .forEach(walk)
  }

  const startNodes = state.nodes
    .filter(node => (incomingCount.get(node.id) ?? 0) === 0)
    .sort((a, b) => a.position.y - b.position.y)

  startNodes.forEach(walk)

  const remaining = state.nodes
    .filter(node => !visited.has(node.id))
    .sort((a, b) => a.position.y - b.position.y)

  remaining.forEach(walk)

  const connectionLabels = (nodeId: string): string => {
    const edges = outgoing.get(nodeId)
    if (!edges || edges.length === 0) return ''
    const labels = edges
      .map(edge => nodeMap.get(edge.target))
      .filter((n): n is Node<AgentFlowNodeData> => Boolean(n))
      .map(node => node.data.title.trim())
      .filter(Boolean)
    return labels.length ? ` → ${labels.join(' | ')}` : ''
  }

  return orderedNodes
    .map((node, index) => {
      const data = node.data
      const header = `${index + 1}. ${labelByFlowType[data.flowType]} — ${data.title.trim()}`
      const detail = data.description.trim() ? ` ${data.description.trim()}` : ''
      const connections = connectionLabels(node.id)
      return `${header}:${detail}${connections}`
    })
    .join('\n')
}

const FlowSidebar = () => {
  const handleDragStart = useCallback((event: DragEvent<HTMLDivElement>, flowType: FlowNodeKind) => {
    event.dataTransfer.setData('application/reactflow', flowType)
    event.dataTransfer.effectAllowed = 'move'
  }, [])

  return (
    <aside className="space-y-4 rounded-xl border border-accentGold/25 bg-primary/45 p-5 shadow-lg backdrop-blur">
      <p className="text-sm text-textOnDark/70">
        Drag blocks to the canvas to model how the agent should reason, call tools, and report outcomes.
      </p>
      <div className="space-y-3">
        {nodeTemplateDescriptors.map(template => (
          <div
            key={template.flowType}
            draggable
            onDragStart={event => handleDragStart(event, template.flowType)}
            className={`cursor-grab rounded-lg border px-3 py-2 text-sm transition hover:scale-[1.01] ${accentByFlowType[template.flowType]}`}
          >
            <p className="font-semibold text-textOnDark">{template.title}</p>
            <p className="text-xs text-textOnDark/70 mt-1 leading-snug">{template.description}</p>
          </div>
        ))}
      </div>
    </aside>
  )
}

const NodeInspector = ({
  node,
  onUpdate,
  onDelete,
}: {
  node: Node<AgentFlowNodeData> | null
  onUpdate: (updates: Partial<AgentFlowNodeData>) => void
  onDelete: () => void
}) => {
  if (!node) {
    return (
      <aside className="h-full rounded-xl border border-accentGold/20 bg-primary/35 p-4 text-sm text-textOnDark/70 shadow-md">
        Select a step to refine its persona, success criteria, and expected outputs.
      </aside>
    )
  }

  const data = node.data

  return (
    <aside className="h-full rounded-xl border border-accentGold/35 bg-primary/45 p-4 space-y-4 shadow-md">
      <div>
        <p className="text-xs uppercase tracking-wide text-textOnDark/50">Editing</p>
        <p className="text-lg font-semibold text-textOnDark">{data.title}</p>
      </div>

      <Input
        label="Title"
        value={data.title}
        onChange={event => onUpdate({ title: event.target.value })}
        placeholder="Name this step"
      />

      <Textarea
        label="Details"
        value={data.description}
        rows={4}
        onChange={event => onUpdate({ description: event.target.value })}
        placeholder="What should happen here?"
      />

      <div className="flex items-center justify-between">
        <span className="text-xs uppercase tracking-wide text-textOnDark/50">{labelByFlowType[data.flowType]}</span>
        <Button variant="ghost" onClick={onDelete}>
          Remove Step
        </Button>
      </div>
    </aside>
  )
}

const FlowPreview = ({ instruction }: { instruction: string }) => (
  <div className="h-full rounded-xl border border-accentGold/20 bg-primary/35 p-4 shadow-md">
    <p className="text-xs uppercase tracking-wide text-textOnDark/50 mb-2">Auto-generated mission brief</p>
    <pre className="whitespace-pre-wrap text-sm text-textOnDark/80 leading-relaxed">{instruction || 'Drag steps into the canvas to build an instruction plan.'}</pre>
  </div>
)

const AgentFlowBuilder = ({ onFlowChange, initialState }: AgentFlowBuilderProps) => {
  const wrapperRef = useRef<HTMLDivElement | null>(null)
  const [reactFlowInstance, setReactFlowInstance] = useState<ReactFlowInstance | null>(null)

  const startingNodes = useMemo(() => initialState?.nodes ?? buildInitialNodes(), [initialState?.nodes])
  const startingEdges = useMemo(() => initialState?.edges ?? buildInitialEdges(), [initialState?.edges])

  const [nodes, setNodes, onNodesChange] = useNodesState<AgentFlowNodeData>(startingNodes)
  const [edges, setEdges, onEdgesChange] = useEdgesState(startingEdges)
  const [selectedNodeId, setSelectedNodeId] = useState<string | null>(startingNodes[0]?.id ?? null)

  const handleConnect = useCallback(
    (connection: Connection | Edge) =>
      setEdges(previous =>
        addEdge(
          {
            ...connection,
            animated: true,
            markerEnd: { type: MarkerType.ArrowClosed, color: '#facc15' },
            style: { stroke: '#facc15', strokeWidth: 2 },
          },
          previous,
        ),
      ),
    [setEdges],
  )

  const handleDragOver = useCallback((event: DragEvent<HTMLDivElement>) => {
    event.preventDefault()
    event.dataTransfer.dropEffect = 'move'
  }, [])

  const handleDrop = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      event.preventDefault()
      const flowType = event.dataTransfer.getData('application/reactflow') as FlowNodeKind
      if (!flowType || !wrapperRef.current || !reactFlowInstance) return

      const bounds = wrapperRef.current.getBoundingClientRect()
      const position = reactFlowInstance.screenToFlowPosition({
        x: event.clientX - bounds.left,
        y: event.clientY - bounds.top,
      })

      const template = nodeTemplateDescriptors.find(descriptor => descriptor.flowType === flowType)
      if (!template) return

      const newNode: Node<AgentFlowNodeData> = {
        id: `node-${Date.now()}-${Math.round(Math.random() * 1000)}`,
        position,
        data: {
          flowType,
          title: template.title,
          description: template.description,
        },
        type: 'agentCard',
      }

      setNodes(previous => [...previous, newNode])
      setSelectedNodeId(newNode.id)
    },
    [reactFlowInstance, setNodes],
  )

  const handleNodeUpdate = useCallback(
    (updates: Partial<AgentFlowNodeData>) => {
      if (!selectedNodeId) return
      setNodes(previous =>
        previous.map(node =>
          node.id === selectedNodeId ? { ...node, data: { ...node.data, ...updates } } : node,
        ),
      )
    },
    [selectedNodeId, setNodes],
  )

  const handleNodeDelete = useCallback(() => {
    if (!selectedNodeId) return
    setNodes(previous => previous.filter(node => node.id !== selectedNodeId))
    setEdges(previous => previous.filter(edge => edge.source !== selectedNodeId && edge.target !== selectedNodeId))
    setSelectedNodeId(null)
  }, [selectedNodeId, setEdges, setNodes])

  const selectedNode = useMemo(
    () => nodes.find(node => node.id === selectedNodeId) ?? null,
    [nodes, selectedNodeId],
  )

  useEffect(() => {
    onFlowChange?.({ nodes, edges })
  }, [edges, nodes, onFlowChange])

  const instructionPreview = useMemo(
    () => generateInstructionFromFlow({ nodes, edges }),
    [nodes, edges],
  )

  return (
    <div className="md:grid md:grid-cols-3 gap-6">
      <div className="md:col-span-1">
        <FlowSidebar />
      </div>
      <div className="md:col-span-2 flex flex-col gap-6">
        <div
          className="rounded-xl border border-accentGold/25 bg-primary/40 p-3 shadow-2xl"
          ref={wrapperRef}
          onDragOver={handleDragOver}
          onDrop={handleDrop}
        >
          <div className="h-[560px] rounded-lg border border-accentGold/20 bg-primary/60">
            <ReactFlow
              nodes={nodes}
              edges={edges}
              onNodesChange={onNodesChange}
              onEdgesChange={onEdgesChange}
              onConnect={handleConnect}
              fitView
              nodeTypes={nodeTypes}
              onInit={setReactFlowInstance}
              onSelectionChange={params => {
                const focused = params.nodes?.[0]
                setSelectedNodeId(focused?.id ?? null)
              }}
              className="text-textOnDark"
              style={{
                backgroundColor: '#080b16',
                backgroundImage:
                  'radial-gradient(circle at 20% 20%, rgba(250,204,21,0.12), transparent 55%), radial-gradient(circle at 80% 30%, rgba(6,182,212,0.12), transparent 60%)',
              }}
            >
              <Background gap={28} color="rgba(250,204,21,0.18)" />
              <MiniMap pannable zoomable maskColor="rgba(8,11,22,0.85)" nodeColor={() => '#facc15'} />
              <Controls />
            </ReactFlow>
          </div>
        </div>
        <div className="grid gap-6 lg:grid-cols-2">
          <NodeInspector node={selectedNode} onUpdate={handleNodeUpdate} onDelete={handleNodeDelete} />
          <FlowPreview instruction={instructionPreview} />
        </div>
      </div>
    </div>
  )
}

export default AgentFlowBuilder
