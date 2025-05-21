# LocAl Web Services (LAWS)

---

Or if you prefer - Lobotomized Amazon Web Services

Developed to locally test with NoSQL DBs and Object Stores, since I'm too broke to afford the real AWS.

---

## API Design

<table>
    <thead>
        <tr>
            <th>Route</th>
            <th>HTTP Method</th>
            <th>Function</th>
            <th>Input</th>
            <th>Output</th>
            <th>Description</th>
        </tr>    
    </thead>
    <tbody>
        <tr><td colspan="6">Database Level CRUD Methods</td></tr>
        <tr>
            <td>/db</td>
            <td>GET</td>
            <td>Database Read</td>
            <td></td>
            <td></td>
            <td>Returns all table names in the database.</td>
        </tr>
        <tr><td colspan="6">Table Level CRUD Methods</td></tr>
        <tr>
            <td>/db/table/{table_name}</td>
            <td>GET</td>
            <td></td>
            <td></td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td>/db/table/{table_name}</td>
            <td>POST</td>
            <td></td>
            <td></td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td>/db/table/{table_name}</td>
            <td>DELETE</td>
            <td></td>
            <td></td>
            <td></td>
            <td></td>
        </tr>
        <tr><td colspan="6">Document Level CRUD Methods</td></tr>
        <tr>
            <td>/db/table/{table_name}/doc</td>
            <td>GET</td>
            <td></td>
            <td></td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td>/db/table/{table_name}/doc</td>
            <td>POST</td>
            <td></td>
            <td></td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td>/db/table/{table_name}/doc</td>
            <td>DELETE</td>
            <td></td>
            <td></td>
            <td></td>
            <td></td>
        </tr>
    </tbody>
</table>

---

## Improvements

1. Add large object store
2. Implement clients
3. Update operations
4. Advanced queries
5. Authentication + API Keys
6. Authorization + Permissions
7. Dashboard