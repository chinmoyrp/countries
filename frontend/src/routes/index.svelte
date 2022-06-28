<script lang="ts">
import { onMount } from "svelte";
    let countries = [[{}]];
    let page = 0;    
    let searchQuery = '';
    const validSearchQuery = new RegExp('[a-z.]+:[^ ]+$');

    let rows = [5, 10, 20, 25, 50, 100, 200];
    let row = rows[0];

    let columns = [
        {id: 0, value: 'default', text: '---'},
        {id: 1, value: 'id', text: 'ID'},
        {id: 2, value: 'name', text: 'Country Name'},
        {id: 3, value: 'capital', text: 'Capital'},
        {id: 4, value: 'population', text: 'Population'},
    ];
    let column = columns[0];

    let orders = [
        {id: 0, value: 'default', text: '---'},
        {id: 1, value: 'asc', text: 'ASC'},
        {id: 2, value: 'desc', text: 'DESC'},
    ];
    let order = orders[0];

    const fetchCountries = (_rows: number, _column: string, _order: string) => {
        const requestOptions = {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ sortby: _column, orderby: _order, rows: _rows })
        };

        fetch('http://localhost:8080/companies', requestOptions)
            .then(response => response.json())
            .then(data => {
               countries = data["countries"];
               page = 0;
            });
    }
    
    const searchCountries = (_search: string, _rows: number, _column: string, _order: string) => {
        const requestOptions = {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ search: _search, sortby: _column, orderby: _order, rows: _rows })
        };

        fetch('http://localhost:8080/search', requestOptions)
            .then(response => response.json())
            .then(data => {
               countries = data["countries"];
               page = 0;
            });
    }

    const nextPage = () => {
        page = page+1;
    }

    const previousPage = () => {
        page = page-1;
    }

    onMount(() => {
        fetchCountries(row, column.value, order.value);
    });

    $: {
        if (searchQuery.length === 0) {
            fetchCountries(row, column.value, order.value);
        } else {
            if (validSearchQuery.test(searchQuery)) {
                searchCountries(searchQuery, row, column.value, order.value)
            }
        }
    }

</script>


<div class="flex flex-col justify-center items-center">
    <div>
        <div class="filter flex flex-row gap-4 mb-4">
            <span>Show
            <select bind:value={row} id="pages">
                {#each rows as r}
                    <option>{r}</option>
                {/each}
            </select>
            rows</span>

            <form on:submit|preventDefault={() => searchCountries(searchQuery, row, column.value, order.value)}>
                <input type="text" placeholder="search..." bind:value={searchQuery}/>
            </form>

            <span>
                <label for="sort-by">Sort By</label>
                <select bind:value={column} id="sort-by">
                    {#each columns as col}
                        <option value={col}>{col.text}</option>
                    {/each}
                </select>
            </span>

            <span>
                <label for="order-by">Order By</label>
                <select bind:value={order} id="order-by">
                    {#each orders as ord}
                        <option value={ord}>{ord.text}</option>
                    {/each}
                </select>
            </span>

        </div>

        <table class="w-full table-auto border border-slate-400">
            <thead class="font-bold">
                <tr>
                    <td>ID</td>
                    <td>Country Name</td>
                    <td>Capital</td>
                    <td>Population</td>
                    <td>Timezone</td>                
                </tr>
            </thead>
            <tbody>
                {#each countries[page] as td}
                <tr>
                    <td>{td.id}</td>
                    <td>{td.name}</td>
                    <td>{td.capital}</td>
                    <td>{td.population}</td>
                    <td>{td.timezone}</td>         
                </tr>
                {/each}
            </tbody>
        </table>

        <div class="flex flex-row gap-4 mt-4 justify-center">
            <button class="nav bg-blue-200 p-2 rounded-lg" on:click={() => page = 0}>&#60&#60</button>
            <button class="nav bg-blue-200 p-2 rounded-lg" on:click={previousPage} disabled={page === 0}>&#60</button>
            <span>
            <select bind:value={page} id="pages">
                {#each {length: countries.length} as _,i}
                    <option value={i}>{i+1}</option>
                {/each}
            </select>
            <label for="pages">/&nbsp{countries.length}</label>
            </span>
            <button class="nav bg-blue-200 p-2 rounded-lg" on:click={nextPage} disabled={page+1 === countries.length}>&#62</button>
            <button class="nav bg-blue-200 p-2 rounded-lg" on:click={() => page = countries.length-1}>&#62&#62</button>
        </div>
    </div>
</div>

<style>
    td {
        padding: 14px;
        border: solid 1px;
    }

    .nav:disabled {
        background: rgb(191, 219, 254, 0.3);
    }
</style>