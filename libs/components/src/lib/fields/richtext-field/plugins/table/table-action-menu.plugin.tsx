import { useLexicalComposerContext } from '@lexical/react/LexicalComposerContext';
import {
	$deleteTableColumn,
	$getElementGridForTableNode,
	$getTableCellNodeFromLexicalNode,
	$getTableColumnIndexFromTableCellNode,
	$getTableNodeFromLexicalNodeOrThrow,
	$getTableRowIndexFromTableCellNode,
	$insertTableColumn,
	$insertTableRow,
	$isTableCellNode,
	$isTableRowNode,
	$removeTableRowAtIndex,
	getTableSelectionFromTableElement,
	HTMLTableElementWithWithTableSelectionState,
	TableCellHeaderStates,
	TableCellNode,
} from '@lexical/table';
import { $getRoot, $getSelection, $isRangeSelection, DEPRECATED_$isGridSelection } from 'lexical';
import * as React from 'react';
import { ReactPortal, useCallback, useEffect, useRef, useState } from 'react';
import { createPortal } from 'react-dom';
import cx from 'classnames/bind';

import styles from './table-action-menu.module.scss'

const cxBind = cx.bind(styles);

type TableCellActionMenuProps = Readonly<{
	contextRef: { current: null | HTMLElement };
	onClose: () => void;
	setIsMenuOpen: (isOpen: boolean) => void;
	tableCellNode: TableCellNode;
}>;

function TableActionMenu({ onClose, tableCellNode: _tableCellNode, setIsMenuOpen, contextRef }: TableCellActionMenuProps) {
	const [editor] = useLexicalComposerContext();
	const dropDownRef = useRef<HTMLDivElement | null>(null);
	const [tableCellNode, updateTableCellNode] = useState(_tableCellNode);
	const [selectionCounts, updateSelectionCounts] = useState({
		columns: 1,
		rows: 1,
	});

	useEffect(() => {
		return editor.registerMutationListener(TableCellNode, (nodeMutations) => {
			const nodeUpdated = nodeMutations.get(tableCellNode.getKey()) === 'updated';

			if (nodeUpdated) {
				editor.getEditorState().read(() => {
					updateTableCellNode(tableCellNode.getLatest());
				});
			}
		});
	}, [editor, tableCellNode]);

	useEffect(() => {
		editor.getEditorState().read(() => {
			const selection = $getSelection();

			if (DEPRECATED_$isGridSelection(selection)) {
				const selectionShape = selection.getShape();

				updateSelectionCounts({
					columns: selectionShape.toX - selectionShape.fromX + 1,
					rows: selectionShape.toY - selectionShape.fromY + 1,
				});
			}
		});
	}, [editor]);

	useEffect(() => {
		const menuButtonElement = contextRef.current;
		const dropDownElement = dropDownRef.current;

		if (menuButtonElement != null && dropDownElement != null) {
			const menuButtonRect = menuButtonElement.getBoundingClientRect();

			dropDownElement.style.opacity = '1';

			dropDownElement.style.left = `${menuButtonRect.left + menuButtonRect.width + window.pageXOffset + 5}px`;

			dropDownElement.style.top = `${menuButtonRect.top + window.pageYOffset}px`;
		}
	}, [contextRef, dropDownRef]);

	useEffect(() => {
		function handleClickOutside(event: MouseEvent) {
			if (
				dropDownRef.current != null &&
				contextRef.current != null &&
				!dropDownRef.current.contains(event.target as Node) &&
				!contextRef.current.contains(event.target as Node)
			) {
				setIsMenuOpen(false);
			}
		}

		window.addEventListener('click', handleClickOutside);

		return () => window.removeEventListener('click', handleClickOutside);
	}, [setIsMenuOpen, contextRef]);

	const clearTableSelection = useCallback(() => {
		editor.update(() => {
			if (tableCellNode.isAttached()) {
				const tableNode = $getTableNodeFromLexicalNodeOrThrow(tableCellNode);
				const tableElement = editor.getElementByKey(tableNode.getKey()) as HTMLTableElementWithWithTableSelectionState;

				if (!tableElement) {
					throw new Error('Expected to find tableElement in DOM');
				}

				const tableSelection = getTableSelectionFromTableElement(tableElement);
				if (tableSelection !== null) {
					tableSelection.clearHighlight();
				}

				tableNode.markDirty();
				updateTableCellNode(tableCellNode.getLatest());
			}

			const rootNode = $getRoot();
			rootNode.selectStart();
		});
	}, [editor, tableCellNode]);

	const insertTableRowAtSelection = useCallback(
		(shouldInsertAfter: boolean) => {
			editor.update(() => {
				const selection = $getSelection();

				const tableNode = $getTableNodeFromLexicalNodeOrThrow(tableCellNode);

				let tableRowIndex;

				if (DEPRECATED_$isGridSelection(selection)) {
					const selectionShape = selection.getShape();
					tableRowIndex = shouldInsertAfter ? selectionShape.toY : selectionShape.fromY;
				} else {
					tableRowIndex = $getTableRowIndexFromTableCellNode(tableCellNode);
				}

				const grid = $getElementGridForTableNode(editor, tableNode);

				$insertTableRow(tableNode, tableRowIndex, shouldInsertAfter, selectionCounts.rows, grid);

				clearTableSelection();

				onClose();
			});
		},
		[editor, tableCellNode, selectionCounts.rows, clearTableSelection, onClose]
	);

	const insertTableColumnAtSelection = useCallback(
		(shouldInsertAfter: boolean) => {
			editor.update(() => {
				const selection = $getSelection();

				const tableNode = $getTableNodeFromLexicalNodeOrThrow(tableCellNode);

				let tableColumnIndex;

				if (DEPRECATED_$isGridSelection(selection)) {
					const selectionShape = selection.getShape();
					tableColumnIndex = shouldInsertAfter ? selectionShape.toX : selectionShape.fromX;
				} else {
					tableColumnIndex = $getTableColumnIndexFromTableCellNode(tableCellNode);
				}

				const grid = $getElementGridForTableNode(editor, tableNode);

				$insertTableColumn(tableNode, tableColumnIndex, shouldInsertAfter, selectionCounts.columns, grid);

				clearTableSelection();

				onClose();
			});
		},
		[editor, tableCellNode, selectionCounts.columns, clearTableSelection, onClose]
	);

	const deleteTableRowAtSelection = useCallback(() => {
		editor.update(() => {
			const tableNode = $getTableNodeFromLexicalNodeOrThrow(tableCellNode);
			const tableRowIndex = $getTableRowIndexFromTableCellNode(tableCellNode);

			$removeTableRowAtIndex(tableNode, tableRowIndex);

			clearTableSelection();
			onClose();
		});
	}, [editor, tableCellNode, clearTableSelection, onClose]);

	const deleteTableAtSelection = useCallback(() => {
		editor.update(() => {
			const tableNode = $getTableNodeFromLexicalNodeOrThrow(tableCellNode);
			tableNode.remove();

			clearTableSelection();
			onClose();
		});
	}, [editor, tableCellNode, clearTableSelection, onClose]);

	const deleteTableColumnAtSelection = useCallback(() => {
		editor.update(() => {
			const tableNode = $getTableNodeFromLexicalNodeOrThrow(tableCellNode);

			const tableColumnIndex = $getTableColumnIndexFromTableCellNode(tableCellNode);

			$deleteTableColumn(tableNode, tableColumnIndex);

			clearTableSelection();
			onClose();
		});
	}, [editor, tableCellNode, clearTableSelection, onClose]);

	const toggleTableRowIsHeader = useCallback(() => {
		editor.update(() => {
			const tableNode = $getTableNodeFromLexicalNodeOrThrow(tableCellNode);

			const tableRowIndex = $getTableRowIndexFromTableCellNode(tableCellNode);

			const tableRows = tableNode.getChildren();

			if (tableRowIndex >= tableRows.length || tableRowIndex < 0) {
				throw new Error('Expected table cell to be inside of table row.');
			}

			const tableRow = tableRows[tableRowIndex];

			if (!$isTableRowNode(tableRow)) {
				throw new Error('Expected table row');
			}

			tableRow.getChildren().forEach((tableCell) => {
				if (!$isTableCellNode(tableCell)) {
					throw new Error('Expected table cell');
				}

				tableCell.toggleHeaderStyle(TableCellHeaderStates.ROW);
			});

			clearTableSelection();
			onClose();
		});
	}, [editor, tableCellNode, clearTableSelection, onClose]);

	const toggleTableColumnIsHeader = useCallback(() => {
		editor.update(() => {
			const tableNode = $getTableNodeFromLexicalNodeOrThrow(tableCellNode);

			const tableColumnIndex = $getTableColumnIndexFromTableCellNode(tableCellNode);

			const tableRows = tableNode.getChildren();

			for (let r = 0; r < tableRows.length; r++) {
				const tableRow = tableRows[r];

				if (!$isTableRowNode(tableRow)) {
					throw new Error('Expected table row');
				}

				const tableCells = tableRow.getChildren();

				if (tableColumnIndex >= tableCells.length || tableColumnIndex < 0) {
					throw new Error('Expected table cell to be inside of table row.');
				}

				const tableCell = tableCells[tableColumnIndex];

				if (!$isTableCellNode(tableCell)) {
					throw new Error('Expected table cell');
				}

				tableCell.toggleHeaderStyle(TableCellHeaderStates.COLUMN);
			}

			clearTableSelection();
			onClose();
		});
	}, [editor, tableCellNode, clearTableSelection, onClose]);

	return createPortal(
		// eslint-disable-next-line jsx-a11y/no-static-element-interactions
		<div
			className={cxBind('o-dropdown')}
			ref={dropDownRef}
			onClick={(e) => {
				e.stopPropagation();
			}}
		>
			<button className={cxBind('o-dropdown__item')} onClick={() => insertTableRowAtSelection(false)}>
				<span>Insert {selectionCounts.rows === 1 ? 'row' : `${selectionCounts.rows} rows`} above</span>
			</button>
			<button className={cxBind('o-dropdown__item')} onClick={() => insertTableRowAtSelection(true)}>
				<span>Insert {selectionCounts.rows === 1 ? 'row' : `${selectionCounts.rows} rows`} below</span>
			</button>
			<hr />
			<button className={cxBind('o-dropdown__item')} onClick={() => insertTableColumnAtSelection(false)}>
				<span>Insert {selectionCounts.columns === 1 ? 'column' : `${selectionCounts.columns} columns`} left</span>
			</button>
			<button className={cxBind('o-dropdown__item')} onClick={() => insertTableColumnAtSelection(true)}>
				<span>Insert {selectionCounts.columns === 1 ? 'column' : `${selectionCounts.columns} columns`} right</span>
			</button>
			<hr />
			<button className={cxBind('o-dropdown__item')} onClick={() => deleteTableColumnAtSelection()}>
				<span>Delete column</span>
			</button>
			<button className={cxBind('o-dropdown__item')} onClick={() => deleteTableRowAtSelection()}>
				<span>Delete row</span>
			</button>
			<button className={cxBind('o-dropdown__item')} onClick={() => deleteTableAtSelection()}>
				<span>Delete table</span>
			</button>
			<hr />
			<button className={cxBind('o-dropdown__item')} onClick={() => toggleTableRowIsHeader()}>
				<span>
					{(tableCellNode.__headerState & TableCellHeaderStates.ROW) === TableCellHeaderStates.ROW ? 'Remove' : 'Add'} row header
				</span>
			</button>
			<button className={cxBind('o-dropdown__item')} onClick={() => toggleTableColumnIsHeader()}>
				<span>
					{(tableCellNode.__headerState & TableCellHeaderStates.COLUMN) === TableCellHeaderStates.COLUMN ? 'Remove' : 'Add'} column header
				</span>
			</button>
		</div>,
		document.body
	);
}

function TableCellActionMenuContainer({ anchorElem }: { anchorElem: HTMLElement }): JSX.Element {
	const [editor] = useLexicalComposerContext();

	const menuButtonRef = useRef(null);
	const menuRootRef = useRef(null);
	const [isMenuOpen, setIsMenuOpen] = useState(false);

	const [tableCellNode, setTableMenuCellNode] = useState<TableCellNode | null>(null);

	const moveMenu = useCallback(() => {
		const menu = menuButtonRef.current;
		const selection = $getSelection();
		const nativeSelection = window.getSelection();
		const activeElement = document.activeElement;

		if (selection == null || menu == null) {
			setTableMenuCellNode(null);
			return;
		}

		const rootElement = editor.getRootElement();

		if ($isRangeSelection(selection) && rootElement !== null && nativeSelection !== null && rootElement.contains(nativeSelection.anchorNode)) {
			const tableCellNodeFromSelection = $getTableCellNodeFromLexicalNode(selection.anchor.getNode());

			if (tableCellNodeFromSelection == null) {
				setTableMenuCellNode(null);
				return;
			}

			const tableCellParentNodeDOM = editor.getElementByKey(tableCellNodeFromSelection.getKey());

			if (tableCellParentNodeDOM == null) {
				setTableMenuCellNode(null);
				return;
			}

			setTableMenuCellNode(tableCellNodeFromSelection);
		} else if (!activeElement) {
			setTableMenuCellNode(null);
		}
	}, [editor]);

	useEffect(() => {
		return editor.registerUpdateListener(() => {
			editor.getEditorState().read(() => {
				moveMenu();
			});
		});
	});

	useEffect(() => {
		const menuButtonDOM = menuButtonRef.current as HTMLButtonElement | null;

		if (menuButtonDOM != null && tableCellNode != null) {
			const tableCellNodeDOM = editor.getElementByKey(tableCellNode.getKey());

			if (tableCellNodeDOM != null) {
				const tableCellRect = tableCellNodeDOM.getBoundingClientRect();
				const menuRect = menuButtonDOM.getBoundingClientRect();
				const anchorRect = anchorElem.getBoundingClientRect();
				menuButtonDOM.style.opacity = '1';
				menuButtonDOM.style.left = `${tableCellRect.right - menuRect.width - 8 - anchorRect.left}px`;
				menuButtonDOM.style.top = `${tableCellRect.top - anchorRect.top + 86}px`;
			} else {
				menuButtonDOM.style.opacity = '0';
			}
		}
	}, [menuButtonRef, tableCellNode, editor, anchorElem]);

	const prevTableCellDOM = useRef(tableCellNode);

	useEffect(() => {
		if (prevTableCellDOM.current !== tableCellNode) {
			setIsMenuOpen(false);
		}

		prevTableCellDOM.current = tableCellNode;
	}, [prevTableCellDOM, tableCellNode]);

	return (
		<div className={cxBind("a-table-cell-action-button__container")} ref={menuButtonRef}>
			{tableCellNode != null && (
				<>
					<button
						className={cxBind("a-table-cell-action-button")}
						onClick={(e) => {
							e.stopPropagation();
							e.preventDefault();
							setIsMenuOpen(!isMenuOpen);
						}}
						ref={menuRootRef}
					>
						<i className="las la-angle-down" />
					</button>
					{isMenuOpen && (
						<TableActionMenu
							contextRef={menuRootRef}
							setIsMenuOpen={setIsMenuOpen}
							onClose={() => setIsMenuOpen(false)}
							tableCellNode={tableCellNode}
						/>
					)}
				</>
			)}
		</div>
	);
}

export const TableCellActionMenuPlugin =({ anchorElem = document.body }: { anchorElem?: HTMLElement }): ReactPortal => {
	return createPortal(<TableCellActionMenuContainer anchorElem={anchorElem} />, anchorElem);
}
